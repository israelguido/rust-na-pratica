use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};

use serde_json::{json, Value};
use sqlx::PgPool;

use crate::{
    dto::usuario_dto::NovoUsuario,
    models::usuario::Usuario,
    repositories::usuario_repository,
};

type ErroApi = (StatusCode, Json<Value>);

// Converte erro SQL em resposta HTTP
fn erro_banco(erro: sqlx::Error) -> ErroApi {
    eprintln!("Erro no banco de dados: {erro}");

    (
        StatusCode::INTERNAL_SERVER_ERROR,
        Json(json!({"erro": "Erro ao acessar banco de dados"})),
    )
}

// GET /usuarios
pub async fn listar_usuarios(
    State(pool): State<PgPool>,
) -> Result<Json<Vec<Usuario>>, ErroApi> {
    let usuarios = usuario_repository::listar(&pool)
        .await
        .map_err(erro_banco)?;

    Ok(Json(usuarios))
}

// POST /usuarios
pub async fn criar_usuario(
    State(pool): State<PgPool>,
    Json(dados): Json<NovoUsuario>,
) -> Result<(StatusCode, Json<Usuario>), ErroApi> {
    let usuario = usuario_repository::criar(&pool, &dados)
        .await
        .map_err(erro_banco)?;

    Ok((StatusCode::CREATED, Json(usuario)))
}

// PUT /usuarios/{id}
pub async fn atualizar_usuario(
    State(pool): State<PgPool>,
    Path(id): Path<i32>,
    Json(dados): Json<NovoUsuario>,
) -> Result<Json<Usuario>, ErroApi> {
    let usuario = usuario_repository::atualizar(&pool, id, &dados)
        .await
        .map_err(erro_banco)?;

    match usuario {
        Some(usuario) => Ok(Json(usuario)),
        None => Err((
            StatusCode::NOT_FOUND,
            Json(json!({"erro": "Usuario nao encontrado"})),
        )),
    }
}

// DELETE /usuarios/{id}
pub async fn deletar_usuario(
    State(pool): State<PgPool>,
    Path(id): Path<i32>,
) -> Result<Json<Value>, ErroApi> {
    let removido = usuario_repository::deletar(&pool, id)
        .await
        .map_err(erro_banco)?;

    if !removido {
        return Err((
            StatusCode::NOT_FOUND,
            Json(json!({"erro": "Usuario nao encontrado"})),
        ));
    }

    Ok(Json(json!({
        "mensagem": "Usuario deletado com sucesso!"
    })))
}