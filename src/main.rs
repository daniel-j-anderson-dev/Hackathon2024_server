use games::tic_tac_toe;

use axum::{
    extract::Path,
    routing::{get, post},
    Extension, Json, Router,
};
use color_eyre::{eyre::eyre, Report};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tokio::net::TcpListener;
use uuid::Uuid;

#[tokio::main]
async fn main() -> Result<(), Report> {
    // setup server
    let mut games = HashMap::<Uuid, TicTacToeSession>::new();

    // set up game
    let ip = "localhost:8080";
    let listener = TcpListener::bind(&ip).await?;
    let router = Router::new()
        .route("/post/login", post(handle_login))
        .route("/get/tictactoe/all", get(handle_get_all_games))
        .route("/get/tictactoe/:game_id", get(handle_get_game))
        .layer(Extension(games));

    // start server
    println!("Listening on http://{ip}");
    axum::serve(listener, router).await?;

    return Ok(());
}

#[derive(Clone, Copy, Serialize, Debug, Deserialize)]
pub struct TicTacToeSession {
    board: tic_tac_toe::Board,
    x_player_id: Option<Uuid>,
    o_player_id: Option<Uuid>,
}
impl TicTacToeSession {
    /// Creates a new game with a player
    pub fn new() -> Self {
        return TicTacToeSession {
            board: tic_tac_toe::Board::default(),
            x_player_id: Some(Uuid::new_v4()),
            o_player_id: None,
        };
    }
    /// This function creates a
    pub fn add_o_player(&mut self) -> Result<(), Report> {
        match self.o_player_id.take() {
            Some(id) => return Err(eyre!(
                "There are already two players in this session of TicTacToe"
            )),
            None => {
                self.o_player_id = Some(Uuid::new_v4());
            }
        };
        return Ok(());
    }
}

async fn handle_get_all_games(
    Extension(games): Extension<HashMap<Uuid, TicTacToeSession>>,
) -> Json<HashMap<Uuid, TicTacToeSession>> {
    return Json(games.clone());
}

async fn handle_get_game(
    Path(game_id): Path<Uuid>,
    Extension(games): Extension<HashMap<Uuid, TicTacToeSession>>,
) -> Json<TicTacToeSession> {
    return Json(games[&game_id]);
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginResponse {
    session_id: Uuid,
    session: TicTacToeSession,
}
impl Default for LoginResponse {
    fn default() -> Self {
        let session_id = Uuid::new_v4();
        return LoginResponse {
            session_id,
            session: TicTacToeSession::new(),
        }
    }
}

async fn handle_login(
    Extension(mut sessions): Extension<HashMap<Uuid, TicTacToeSession>>,
) -> Json<LoginResponse> {
    println!("{sessions:#?}");
    for (session_id, session) in sessions.iter_mut() {
        if let Ok(()) = session.add_o_player() {
            return Json(LoginResponse {
                session_id: *session_id,
                session: *session,
            });
        }
    }
    let login_response = LoginResponse::default();
    sessions.insert(login_response.session_id.clone(), login_response.session.clone());
    return Json(login_response);
}
