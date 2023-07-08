use axum::{routing::get, Router};
use ngrok::prelude::*;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let app = Router::new().route("/", get(|| async { "Hello from ngrok-rust!" }));

    let listener = ngrok::Session::builder()
        .authtoken_from_env()
        .connect()
        .await?
        .http_endpoint()
        .listen()
        .await?;
    println!("Ingress URL: {:?}", listener.url());
    axum::Server::builder(listener)
        .serve(app.into_make_service())
        .await?;
    Ok(())
}