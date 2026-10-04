use axum::{
        routing::{get, post, put},
        Json, 
        Router,
        extract::{Path, State, Request},
        middleware::{self, Next},
        response::Response,
        http::{StatusCode, header},
    };

use serde_json::{json, Value};
use serde::{ Deserialize, Serialize };

/** use com a lib de banco de dados postgre */
use sqlx::{ FromRow, PgPool, postgres::PgPoolOptions };

/** import o nosso .env */
use dotenvy::dotenv;

/** JWT use */
use jsonwebtoken::{
    encode,
    decode,
    Header,
    Validation,
    EncodingKey,
    DecodingKey,
};
use chrono::{Utc, Duration};


#[derive(Debug, Serialize, FromRow)]
pub struct Usuario {
    pub id: i32,
    pub nome: String,
    pub email: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NovoUsuario {
    pub nome: String,
    pub email: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    exp: usize,
}

#[derive(Debug, Deserialize)]
struct Login {
    email: String,
    senha: String,
}

/** gerar token */
fn gerar_token(email: &str) -> String {

    let secret = std::env::var("JWT_SECRET").expect("JWT_SECRET não existe no .env");
    let expiracao = Utc::now() + Duration::hours(1);

    let claims = Claims {
        sub: email.to_string(),
        exp: expiracao.timestamp() as usize,
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes())
    ).unwrap()

}

/** middleware validação */
pub async fn validar_token(
    request: Request,
    next: Next,
) -> Result<Response, StatusCode> {

    let auth_header = request
        .headers()
        .get(header::AUTHORIZATION)
        .and_then(|header| header.to_str().ok());

    let auth_header = match auth_header {
        Some(valor) => valor,
        None => return Err(StatusCode::UNAUTHORIZED),
    };

    let token = match auth_header.strip_prefix("Bearer ") {
        Some(token) => token,
        None => return Err(StatusCode::UNAUTHORIZED),
    };

    let secret = std::env::var("JWT_SECRET").expect("erro ao tentar ler o secret no env");

    decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::default(),
    )
    .map_err(|_| StatusCode::UNAUTHORIZED)?;

    Ok(next.run(request).await)

}

/** metodo de login */
async fn login(
    Json(login): Json<Login>
) -> Json<Value>
{
    if login.email == "admin@email.com" && login.senha == "123456"
    {
        let token = gerar_token(&login.email);

        Json(json!({
            "token": token
        }))

    } else {

        Json(json!({
            "erro": "Usuário não existe!"
        }))
    }

}

#[tokio::main]
async fn main() {

    dotenv().ok();

    let pool = PgPoolOptions::new()
    .max_connections(5)
    .connect(&std::env::var("DATABASE_URL").unwrap())
    .await
    .expect("Tivemos algum problema na conexão com o postgre");

    let app = Router::new()
        .route("/", get(|| async {
            "API Rust + SQLx"
        }))
        .route(
            "/usuarios",
            get(listar_usuarios).post(criar_usuario),
        )
        .route(
            "/usuarios/{id}", 
            put(atualizar_usuario).delete(deletar_usuario)
        )
        .route_layer(
            middleware::from_fn(validar_token)
        )
        .route(
            "/login",
            post(login)
        )
        .with_state(pool);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await.unwrap();

    println!("🚀 API rodando em http://127.0.0.1:3000");

    axum::serve(listener, app).await.unwrap();
}

/** listar usuarios */
pub async fn listar_usuarios(
    State(pool): State<PgPool>,
) -> Json<Vec<Usuario>> {

    let usuarios = sqlx::query_as::<_, Usuario>(
        "SELECT * FROM usuarios"
    ).fetch_all(&pool).await.unwrap();

    Json(usuarios)
    
}

/** criar usuario */
pub async fn criar_usuario(
    State(pool): State<PgPool>,
    Json(usuario): Json<NovoUsuario>
) -> Json<Value> {

    sqlx::query("INSERT INTO usuarios (nome, email) VALUES ($1, $2)")
    .bind(&usuario.nome)
    .bind(&usuario.email)
    .execute(&pool)
    .await
    .unwrap();

    Json(json!({
        "mensagem": "Usuário criado com sucesso!"
    }))

}

/** atualizar usuario */
pub async fn atualizar_usuario(
    State(pool): State<PgPool>,
    Path(id): Path<i32>,
    Json(usuario): Json<NovoUsuario>
) -> Json<Value>
{
    sqlx::query(
        "UPDATE usuarios SET nome = $1, email = $2 WHERE id = $3"
    )
    .bind(&usuario.nome)
    .bind(&usuario.email)
    .bind(id)
    .execute(&pool)
    .await
    .unwrap();

    Json(json!({
        "mensagem": "Usuario atualizado com sucesso!"
    }))
}

/** deletar usuarios */
pub async fn deletar_usuario(
    State(pool): State<PgPool>,
    Path(id): Path<i32>,
) -> Json<Value>
{
    sqlx::query(
        "DELETE FROM usuarios WHERE id = $1"
    )
    .bind(id)
    .execute(&pool)
    .await
    .unwrap();

    Json(json!({
        "mensagem": "Usuário deletado com sucesso!"
    }))
}