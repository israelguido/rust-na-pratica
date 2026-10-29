use axum::{http::StatusCode, Json};
use serde_json::{json, Value};

use crate::{
    dto::auth_dto::{LoginRequest, LoginResponse},
    utils::jwt,
};

// Login demonstrativo com credenciais do .env
pub async fn login(
    Json(dados): Json<LoginRequest>,
) -> Result<Json<LoginResponse>, (StatusCode, Json<Value>)> {
    let email = std::env::var("ADMIN_EMAIL")
        .map_err(|_| erro_interno())?;

    let senha = std::env::var("ADMIN_PASSWORD")
        .map_err(|_| erro_interno())?;

    if dados.email != email || dados.senha != senha {
        return Err((
            StatusCode::UNAUTHORIZED,
            Json(json!({"erro": "Credenciais invalidas"})),
        ));
    }

    let token = jwt::gerar_token(&dados.email)
        .map_err(|_| erro_interno())?;

    Ok(Json(LoginResponse { token }))
}

fn erro_interno() -> (StatusCode, Json<Value>) {
    (
        StatusCode::INTERNAL_SERVER_ERROR,
        Json(json!({"erro": "Erro interno do servidor"})),
    )
}