use serde::{Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, FromRow)]
pub struct Usuario {
    pub id: i32,
    pub nome: String,
    pub email: String,
}