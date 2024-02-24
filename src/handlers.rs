use axum::{
    debug_handler,
    extract::State,
    http::StatusCode,
    Json,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use games::tic_tac_toe::Cell;
use crate::{SessionCapacity, Sessions, TicTacToeSession, TicTacToeUpdate};

#[debug_handler]
pub async fn handle_root() -> &'static str {
    "Endpoints are\nGET /get\nPUT /update\nPOST /join\nPUT /leave"
}

#[debug_handler]
pub async fn handle_join(State(sessions): State<Sessions>) -> Json<(Uuid, TicTacToeSession)> {
    let mut sessions = sessions.lock().await;

    // loop through existing session and add a player where available
    for (session_id, session) in sessions.iter_mut() {
        match session.capacity() {
            SessionCapacity::Empty | SessionCapacity::OnlyPlayer2 => {
                session.add_player1();
                return Json((*session_id, *session,));
            }
            SessionCapacity::OnlyPlayer1 => {
                session.add_player2();
                return Json((*session_id, *session));
            }
            SessionCapacity::Full => {}
        }
    }

    // if all of the active sessions were full then create a new one
    let new_session_id = Uuid::new_v4();
    let mut new_session = TicTacToeSession::default();
    new_session.add_player1();
    sessions.insert(new_session_id, new_session);
    return Json((new_session_id, new_session));
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SessionRequest {
    session_id: Uuid,
}
#[debug_handler]
pub async fn handle_get_game(
    State(sessions): State<Sessions>,
    Json(SessionRequest { session_id }): Json<SessionRequest>,
) -> Result<Json<(Uuid, TicTacToeSession)>, StatusCode> {
    let sessions = sessions.lock().await;
    return match sessions.get(&session_id) {
        Some(&session) => Ok(Json((session_id, session))),
        None => Err(StatusCode::NOT_FOUND),
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub enum UpdateError {
    InvalidSessionId,
    SessionIsEmpty,
    ExpectedPlayer2Id,
    ExpectedPlayer1Id,
    InvalidPlayerId,
    CellAlreadyFull,
}

#[debug_handler]
/// This handler function will update a specific game.
/// - State(`sessions`): This is the backbone data of the server
/// - Json(`update`): This must have a valid session [Uuid], and a correspondingly valid player [Uuid]
pub async fn handle_game_update(
    State(sessions): State<Sessions>,
    Json(update): Json<TicTacToeUpdate>,
) -> Result<Json<(Uuid, TicTacToeSession)>, Json<UpdateError>> {
    let mut sessions = sessions.lock().await;

    let session = sessions
        .get_mut(&update.session_id)
        .ok_or_else(|| UpdateError::InvalidSessionId)?;

    if !session.capacity().is_full() {
        return Err(Json(UpdateError::SessionIsEmpty));
    }
    let player1_id = session.player1_id().expect("full capacity means player1 is Some");
    let player2_id = session.player2_id().expect("full capacity means player2 is Some");

    let cell_value = if player1_id == update.player_id {
        if session.state.is_x_turn {
            Cell::X
        } else {
            return Err(Json(UpdateError::ExpectedPlayer2Id));
        }
    } else if player2_id == update.player_id {
        if !session.state.is_x_turn {
            Cell::O
        } else {
            return Err(Json(UpdateError::ExpectedPlayer1Id));
        }
    } else {
        return Err(Json(UpdateError::InvalidPlayerId));
    };

    if !session.state.board.get_cell(update.cell_index).is_empty() {
        return Err(Json(UpdateError::CellAlreadyFull));
    }

    session.state.board.set_cell(update.cell_index, cell_value);
    session.state.is_x_turn = !session.state.is_x_turn;
    session.state.winner = session.state.board.get_winner();

    return Ok(Json((update.session_id, *session)));
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LeaveRequest {
    session_id: Uuid,
    player_id: Option<Uuid>,
}

#[debug_handler]
pub async fn handle_leave(
    State(sessions): State<Sessions>,
    Json(LeaveRequest { session_id, player_id }): Json<LeaveRequest>,
) -> Result<(), StatusCode> {
    let mut sessions = sessions.lock().await;

    let is_session_over = {
        let session = sessions
            .get_mut(&session_id)
            .ok_or_else(|| StatusCode::NOT_FOUND)?;

        if session.capacity().is_empty() {
            return Err(StatusCode::FORBIDDEN);
        }

        if player_id == session.player1_id() {
            session.remove_player1();
        } else if player_id == session.player2_id() {
            session.remove_player2();
        } else {
            return Err(StatusCode::BAD_REQUEST);
        }

        session.capacity().is_empty()
    };

    if is_session_over {
        sessions
            .remove(&session_id)
            .expect("If the session id was invalid we would have returned already");
    }

    return Ok(());
}
