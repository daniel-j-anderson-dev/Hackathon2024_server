use axum::{routing::get, Router};
use color_eyre::{eyre::eyre, Report};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> Result<(), Report> {
    let ip = "localhost:8080";

    let listener = TcpListener::bind(&ip).await?;

    let router = Router::new().route("/", get(hello_world));

    println!("Listening on http://{ip}");
    axum::serve(listener, router).await?;

    return Ok(());
}

async fn hello_world() -> &'static str {
    return "Hello, 🌎!";
}
