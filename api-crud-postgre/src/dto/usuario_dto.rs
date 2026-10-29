use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct NovoUsuario {
    pub nome: String,
    pub email: String,
}
