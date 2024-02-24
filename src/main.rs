pub mod handlers;
pub mod session;

use axum::{
    routing::{get, post},
    Router,
};
use color_eyre::Report;
use std::{collections::HashMap, sync::Arc};
use tokio::{net::TcpListener, sync::Mutex};
use uuid::Uuid;

use crate::{handlers::*, session::TicTacToeSession};

#[derive(Debug, Clone)]
pub struct AppData {
    pub data: Arc<Mutex<HashMap<Uuid, TicTacToeSession>>>,
}

#[tokio::main]
async fn main() -> Result<(), Report> {
    // setup server
    let mut sessions = AppData {
        data: Arc::new(Mutex::new(HashMap::new())),
    };

    // set up game
    let ip = "localhost:8080";
    let listener = TcpListener::bind(&ip).await?;
    let router = Router::new()
        .route("/ttt/:session_id", get(handle_get_game_by_id).post(handle_game_update))
        .route("/ttt/join", post(handle_join))
        .with_state(sessions);

    // start server
    println!("Listening on http://{ip}");
    axum::serve(listener, router).await?;

    return Ok(());
}
