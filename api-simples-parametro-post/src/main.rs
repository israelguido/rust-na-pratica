use axum::{
        routing::{get, post},
        Json, 
        Router,
        extract::Path
    };

use serde_json::{json, Value};

use serde::Deserialize;

#[derive(Deserialize)]
struct Usuario {
    nome: String,
    idade: u32
}

//rota POST
async fn criar_usuario(Json(usuario): Json<Usuario>) -> Json<Value> {
    Json(json!({
        "mensagem": format!("Usuário {} encontrado com sucesso!", usuario.nome),
        "Idade": usuario.idade
    }))
}

// rota GET
async fn hello(Path(name): Path<String>) -> Json<Value> {
    Json(json!({
        "message": format!("Hello, {}!", name)
    }))    
}

#[tokio::main] // Inicia o servidor Tokio
async fn main() {
    let app = Router::new()
    .route("/hello/{name}", get(hello))
    .route("/usuario", post(criar_usuario));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await.unwrap(); // inicia o servidor na porta 3000
    axum::serve(listener, app).await.unwrap();
}

/*
curl --location 'http://127.0.0.1:3000/usuario' \
--header 'Content-Type: application/json' \
--data '{"nome":"BRuna","idade":25}'
*/