use axum::{
    middleware,
    routing::{get, post},
    Router,
};

use sqlx::PgPool;

use crate::{
    controllers::{
        auth_controller,
        usuario_controller,
    },
    middleware::auth_middleware,
};

pub fn criar_rotas(pool: PgPool) -> Router {
    // Rotas que exigem JWT
    let rotas_protegidas = Router::new()
        .route(
            "/usuarios",
            get(usuario_controller::listar_usuarios)
                .post(usuario_controller::criar_usuario),
        )
        .route(
            "/usuarios/{id}",
            axum::routing::put(
                usuario_controller::atualizar_usuario
            )
            .delete(usuario_controller::deletar_usuario),
        )
        .route_layer(
            middleware::from_fn(
                auth_middleware::validar_jwt
            )
        );

    // Rotas públicas e protegidas
    Router::new()
        .route(
            "/",
            get(|| async { "API Rust + SQLx" }),
        )
        .route(
            "/login",
            post(auth_controller::login),
        )
        .merge(rotas_protegidas)
        .with_state(pool)
}