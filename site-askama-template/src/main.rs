use askama::Template;
use askama_web::WebTemplate;

use axum::{
    //response::Html, // Retorna a reposta em HTML
    routing::get, //Registra as rotas
    Router, //Organiza os endereços 
};

//Template inicio
#[derive(Template, WebTemplate)]
#[template(path = "inicio.html")]

struct InicioTemplate {
    titulo: String,
    mensagem: String,
}

//Template inicio
#[derive(Template, WebTemplate)]
#[template(path = "sobre.html")]

struct SobreTemplate {
    titulo: String,
    mensagem: String,
}


//inicio
async fn inicio() -> InicioTemplate {
    InicioTemplate{
        titulo: "Meu primeiro template Inicio.html".to_string(),
        mensagem: "Esta página foi enviada pelo Axum usando Askama!".to_string()
    }
}

//sobre
async fn sobre() -> SobreTemplate {
    SobreTemplate{
        titulo: "Titulo Sobre".to_string(),
        mensagem: "Sobre Askama!".to_string()
    }
}

#[tokio::main]
async fn main() {
    
    let app = Router::new()
    .route("/", get(inicio))
    .route("/sobre", get(sobre));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await.expect("Não foi possivel abrir a porta 3000");

    println!("Site disponivel em http://127.0.0.1:3000");

    axum::serve(listener, app).await.expect("Error ao executar o servidor");
}


