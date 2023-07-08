use axum::{routing::get, Router};
use ngrok::prelude::*;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // build our application with a route
    let app = Router::new().route("/", get(|| async { "Hello from ngrok-rust!" }));

    // listen on localhost:8000
    // axum::Server::bind(&"0.0.0.0:8000".parse().unwrap())
    //  .serve(app.into_make_service())
    //  .await?;
    // Ok(())

    // listen on ngrok ingress (i.e. https://myapp.ngrok.dev)
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