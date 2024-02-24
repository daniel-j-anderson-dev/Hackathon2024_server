pub mod handlers;
pub mod session;

use axum::{
    routing::{get, post, put},
    Router,
};
use color_eyre::Report;
use std::{collections::HashMap, sync::Arc};
use tokio::{net::TcpListener, sync::Mutex};
use uuid::Uuid;

use crate::{handlers::*, session::TicTacToeSession};

pub type Sessions = Arc<Mutex<HashMap<Uuid, TicTacToeSession>>>;

#[tokio::main]
async fn main() -> Result<(), Report> {
    // setup server
    let sessions = Arc::new(Mutex::new(HashMap::new()));

    // set up game
    let ip = "localhost:8080";
    let listener = TcpListener::bind(&ip).await?;
    let router = Router::new()
        .route("/ttt/get", get(handle_get_game))
        .route("/ttt/update", put(handle_game_update))
        .route("/ttt/join", post(handle_join))
        .route("/ttt/leave", put(handle_leave))
        .with_state(sessions);

    // start server
    println!("Serving TicTacToe at http://{ip}");
    axum::serve(listener, router).await?;

    return Ok(());
}
