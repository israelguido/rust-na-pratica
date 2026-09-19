use axum::{routing::get, Json, Router};
use serde_json::{json, Value};

async fn hello() -> Json<Value> {
    Json(json!({
        "message": "Hello, World!"
    }))    
}

#[tokio::main] // Inicia o servidor Tokio
async fn main() {
    let app = Router::new().route("/hello", get(hello));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await.unwrap(); // inicia o servidor na porta 3000
    axum::serve(listener, app).await.unwrap();
}