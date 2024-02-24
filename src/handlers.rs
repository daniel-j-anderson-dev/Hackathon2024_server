use axum::{
    debug_handler,
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use games::tic_tac_toe::Cell;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    session::{SessionCapacity, TicTacToeSession, TicTacToeUpdate},
    AppData,
};

#[derive(Debug, Serialize, Deserialize)]
pub struct SessionResponse {
    session_id: Uuid,
    session: TicTacToeSession,
}
#[debug_handler]
pub async fn handle_join(State(sessions): State<AppData>) -> Json<SessionResponse> {
    let mut sessions = sessions.data.lock().await;

    // loop through existing session and add a player where available
    for (session_id, session) in sessions.iter_mut() {
        match session.capacity() {
            SessionCapacity::Empty | SessionCapacity::OnlyPlayer2 => {
                session.add_player1();
                return Json(SessionResponse {
                    session_id: *session_id,
                    session: *session,
                });
            }
            SessionCapacity::OnlyPlayer1 => {
                session.add_player2();
                return Json(SessionResponse {
                    session_id: *session_id,
                    session: *session,
                });
            }
            SessionCapacity::Full => {}
        }
    }

    // if all of the active sessions were full then create a new one
    let new_session_id = Uuid::new_v4();
    let mut new_session = TicTacToeSession::default();
    new_session.add_player1();
    sessions.insert(new_session_id, new_session);
    return Json(SessionResponse {
        session_id: new_session_id,
        session: new_session,
    });
}

#[debug_handler]
pub async fn handle_get_game_by_id(
    State(sessions): State<AppData>,
    Path(session_id): Path<Uuid>,
) -> Json<SessionResponse> {
    let sessions = sessions.data.lock().await;
    return Json(SessionResponse {
        session: sessions[&session_id],
        session_id,
    });
}

#[debug_handler]
pub async fn handle_game_update(
    State(sessions): State<AppData>,
    Json(update): Json<TicTacToeUpdate>,
) -> Result<Json<SessionResponse>, StatusCode> {
    let mut sessions = sessions.data.lock().await;

    let session = sessions
        .get_mut(&update.session_id)
        .ok_or_else(|| StatusCode::NOT_FOUND)?;

    if !session.capacity().is_full() {
        return Err(StatusCode::FORBIDDEN);
    }

    let cell_value = if session
        .player1_id()
        .expect("full capacity means player1 is Some")
        == update.player_id
    {
        Cell::X
    } else if session
        .player2_id()
        .expect("full capacity means player2 is Some")
        == update.player_id
    {
        Cell::O
    } else {
        return Err(StatusCode::NOT_MODIFIED);
    };

    session.state.board.set_cell(update.cell_index, cell_value);

    return Ok(Json(SessionResponse {
        session: *session,
        session_id: update.session_id,
    }));
}
