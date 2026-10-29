use sqlx::PgPool;

use crate::{
    dto::usuario_dto::NovoUsuario,
    models::usuario::Usuario,
};

// Lista todos os usuários
pub async fn listar(
    pool: &PgPool,
) -> Result<Vec<Usuario>, sqlx::Error> {
    sqlx::query_as::<_, Usuario>(
        "SELECT id, nome, email FROM usuarios ORDER BY id"
    )
    .fetch_all(pool)
    .await
}

// Cria um usuário e retorna o registro criado
pub async fn criar(
    pool: &PgPool,
    usuario: &NovoUsuario,
) -> Result<Usuario, sqlx::Error> {
    sqlx::query_as::<_, Usuario>(
        "INSERT INTO usuarios (nome, email)
         VALUES ($1, $2)
         RETURNING id, nome, email"
    )
    .bind(&usuario.nome)
    .bind(&usuario.email)
    .fetch_one(pool)
    .await
}

// Atualiza um usuário existente
pub async fn atualizar(
    pool: &PgPool,
    id: i32,
    usuario: &NovoUsuario,
) -> Result<Option<Usuario>, sqlx::Error> {
    sqlx::query_as::<_, Usuario>(
        "UPDATE usuarios
         SET nome = $1, email = $2
         WHERE id = $3
         RETURNING id, nome, email"
    )
    .bind(&usuario.nome)
    .bind(&usuario.email)
    .bind(id)
    .fetch_optional(pool)
    .await
}

// Remove um usuário pelo ID
pub async fn deletar(
    pool: &PgPool,
    id: i32,
) -> Result<bool, sqlx::Error> {
    let resultado = sqlx::query(
        "DELETE FROM usuarios WHERE id = $1"
    )
    .bind(id)
    .execute(pool)
    .await?;

    Ok(resultado.rows_affected() > 0)
}