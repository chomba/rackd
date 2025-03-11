use std::fs;
use axum::{extract::Path, response::{Html, IntoResponse}, routing::get, Router};
use tower_http::services::ServeDir;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .nest_service("/ui", ServeDir::new("ui"))
        .route("/{*path}", get(index))
        .route("/", get(index));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}        

async fn index() -> impl IntoResponse {
    match fs::read_to_string("ui/index.html") {
        Ok(content) => Html(content),
        Err(e) => Html(String::from("Load 404 HTML Template"))
    }
}