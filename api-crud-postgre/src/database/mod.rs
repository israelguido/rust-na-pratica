use sqlx::{PgPool, postgres::PgPoolOptions };

// Cria o pool de conexões com o PostgreSQL
pub async fn conectar() -> Result<PgPool, sqlx::Error> {

    let database_url = std::env::var("DATABASE_URL").expect("falta string de conexão no ENV");

    PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
}