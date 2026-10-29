use chrono::{Duration, Utc};

use jsonwebtoken::{
    decode,
    encode,
    Algorithm,
    DecodingKey,
    EncodingKey,
    Header,
    Validation,
};

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
}

// Gera um JWT válido por uma hora
pub fn gerar_token(
    email: &str,
) -> Result<String, jsonwebtoken::errors::Error> {
    let secret = std::env::var("JWT_SECRET")
        .expect("Configure JWT_SECRET no arquivo .env");

    let claims = Claims {
        sub: email.to_string(),
        exp: (Utc::now() + Duration::hours(1)).timestamp() as usize,
    };

    encode(
        &Header::new(Algorithm::HS256),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )
}

// Verifica assinatura e expiração do token
pub fn validar_token(
    token: &str,
) -> Result<Claims, jsonwebtoken::errors::Error> {
    let secret = std::env::var("JWT_SECRET")
        .expect("Configure JWT_SECRET no arquivo .env");

    let validation = Validation::new(Algorithm::HS256);

    let dados = decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &validation,
    )?;

    Ok(dados.claims)
}