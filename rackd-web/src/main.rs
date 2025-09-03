use std::fs;
use axum::{extract::Path, response::{Html, IntoResponse}, routing::get, Router};
use serde::{Deserialize, Serialize};
use serde_json;
use tower_http::services::ServeDir;

#[derive(Debug, Serialize, Deserialize)]
struct Person {
    name: String
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "mode", content = "person")]
#[serde(rename_all = "snake_case")] 
enum Mac {
    Auto,
    Static(Person)
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Container {
    id: u32,
    mac: Mac
}
// https://stackoverflow.com/questions/59167416/how-can-i-deserialize-an-enum-when-the-case-doesnt-match
// fn main() {
//     let s = r#"{"id":123,"mac":{"mode":"static","person":{"name":"Bob"}}}"#;
//     let s2 = r#"{"id": 235, "mac":{"mode":"autox"}}"#;
//     let x: Container = serde_json::from_str(&s.to_ascii_lowercase()).unwrap();
//     let x2: Container = serde_json::from_str(&s2.to_ascii_lowercase()).unwrap();
//     // let x = Mac::Static(Person { name: "Bob".into() });
//     // let s = serde_json::to_string(&x).unwrap();
//     println!("{x:?}");
//     println!("{x2:?}");
// }

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