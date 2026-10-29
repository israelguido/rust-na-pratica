mod controllers;
mod database;
mod dto;
mod utils;
mod middleware;
mod models;
mod repositories;
mod routes;

use dotenvy::dotenv;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    dotenv().ok();

    //configurar o provedor JWT
    jsonwebtoken::crypto::rust_crypto::DEFAULT_PROVIDER
        .install_default()
        .expect("Falha ao instalar provedor JWT");

    // Abre conexão com PostgreSQL
    let pool = database::conectar().await?;

    // Registra as rotas da API
    let app = routes::criar_rotas(pool);

    // Configura endereço do servidor
    let host = std::env::var("HOST")
        .unwrap_or_else(|_| "127.0.0.1".to_string());

    let port = std::env::var("PORT")
        .unwrap_or_else(|_| "3000".to_string());

    let endereco = format!("{host}:{port}");

    // Inicia servidor Axum
    let listener = tokio::net::TcpListener::bind(&endereco).await?;

    println!("API rodando em http://{endereco}");

    axum::serve(listener, app).await?;

    Ok(())
}