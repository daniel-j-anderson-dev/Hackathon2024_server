pub mod handlers;
pub mod session;


use axum::{
    routing::{get, post, put},
    Router,
};
use color_eyre::Report;
use std::{sync::Arc, collections::HashMap};
use tokio::{net::TcpListener, sync::Mutex};
use uuid::Uuid;

pub use crate::{handlers::*, session::*};

pub type Sessions = Arc<Mutex<HashMap<Uuid, TicTacToeSession>>>;

#[tokio::main]
async fn main() -> Result<(), Report> {
    // setup server
    let sessions = Arc::new(Mutex::new(HashMap::new()));

    // set up game
    let ip = "localhost:8080";
    let listener = TcpListener::bind(&ip).await?;
    let router = Router::new()
        .route("/", get(handle_root))
        .route("/get", get(handle_get_game))
        .route("/update", put(handle_game_update))
        .route("/join", post(handle_join))
        .route("/leave", put(handle_leave))
        .with_state(sessions);

    // start server
    println!("Serving TicTacToe at http://{ip}");
    axum::serve(listener, router).await?;

    return Ok(());
}
