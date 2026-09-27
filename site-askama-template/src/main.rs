use askama::Template;
use askama_web::WebTemplate;

use axum::{
    response::Html, // Retorna a reposta em HTML
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


//inicio
async fn inicio() -> InicioTemplate {
    InicioTemplate{
        titulo: "Meu primeiro template".to_string(),
        mensagem: "Esta página foi enviada pelo Axum usando Askama!".to_string()
    }
}

//sobre
async fn sobre() -> Html<&'static str> {
    Html(r#"
    <!DOCTYPE html>
<html lang="pt-BR">
<head>
    <!-- Configura a codificação dos caracteres. -->
    <meta charset="UTF-8">

    <!-- Configura a exibição em celulares. -->
    <meta name="viewport" content="width=device-width, initial-scale=1">

    <!-- Define o título da aba. -->
    <title>Sobre o site</title>
</head>
<body>
    <!-- Agrupa o conteúdo principal. -->
    <main>
        <!-- Título da página. -->
        <h1>Sobre o site</h1>

        <!-- Texto explicativo. -->
        <p>Este é um exemplo de site feito com Rust e Axum.</p>

        <!-- Solicita a página inicial ao clicar. -->
        <a href="/">Voltar ao início</a>
    </main>
</body>
    "#)
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


