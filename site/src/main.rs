use axum::{
    response::Html, // Retorna a reposta em HTML
    routing::get, //Registra as rotas
    Router, //Organiza os endereços 
};

//inicio
async fn inicio() -> Html<&'static str> {
    Html(r#"
    <!DOCTYPE html>
<html lang="pt-BR">
<head>
    <!-- Permite exibir acentos corretamente. -->
    <meta charset="UTF-8">

    <!-- Ajusta a página à largura da tela, inclusive no celular. -->
    <meta name="viewport" content="width=device-width, initial-scale=1">

    <!-- Texto exibido na aba do navegador. -->
    <title>Meu site com Rust</title>

    <!-- CSS: define a aparência da página. -->
    <style>
        body {
            /* Define a fonte do site. */
            font-family: Arial, sans-serif;

            /* Define a cor de fundo. */
            background: #f3f4f6;

            /* Define a cor dos textos. */
            color: #1f2937;

            /* Remove a margem padrão do navegador. */
            margin: 0;

            /* Cria espaço entre o conteúdo e as bordas da tela. */
            padding: 24px;
        }

        main {
            /* Limita a largura do conteúdo em telas grandes. */
            max-width: 720px;

            /* Centraliza o bloco horizontalmente. */
            margin: 40px auto;

            /* Define o fundo branco do bloco. */
            background: white;

            /* Cria espaço dentro do bloco. */
            padding: 24px;

            /* Arredonda os cantos. */
            border-radius: 12px;
        }

        a {
            /* Define a cor dos links. */
            color: #2563eb;
        }
    </style>
</head>
<body>
    <!-- Conteúdo principal da página. -->
    <main>
        <!-- Título principal visível. -->
        <h1>Meu primeiro site com Rust</h1>

        <!-- Parágrafo de apresentação. -->
        <p>Esta página foi enviada por um servidor Axum!</p>

        <!-- Ao clicar, o navegador solicita a rota /sobre. -->
        <a href="/sobre">Conheça o site</a>
    </main>
</body>
</html>
    "#)
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


