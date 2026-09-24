use axum::{
        routing::get,
        Json, 
        Router,
        extract::Path
    };
use serde_json::{json, Value};

async fn hello(Path(name): Path<String>) -> Json<Value> {
    Json(json!({
        "message": format!("Hello, {}!", name)
    }))    
}

#[tokio::main] // Inicia o servidor Tokio
async fn main() {
    let app = Router::new()
    .route("/hello/{name}", get(hello));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await.unwrap(); // inicia o servidor na porta 3000
    axum::serve(listener, app).await.unwrap();
}