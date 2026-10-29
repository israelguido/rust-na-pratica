use axum::{
    extract::Request,
    http::{header, StatusCode},
    middleware::Next,
    response::Response,
};

use crate::utils::jwt;

pub async fn validar_jwt(
    request: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    // Lê o cabeçalho Authorization
    let token = request
        .headers()
        .get(header::AUTHORIZATION)
        .and_then(|valor| valor.to_str().ok())
        .and_then(|valor| valor.strip_prefix("Bearer "))
        .ok_or(StatusCode::UNAUTHORIZED)?;

    // Verifica a autenticidade e validade do JWT
    jwt::validar_token(token)
        .map_err(|_| StatusCode::UNAUTHORIZED)?;

    // Permite continuar para o controller
    Ok(next.run(request).await)
}