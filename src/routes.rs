use axum::{Router, routing::get};

use crate::{error, get_users, model::usuario::Cliente, traits::crud::Get};

pub fn create_router() -> Router {
    Router::new()
        .route("/health", get(|| async { "Es servidor está saludable" }))
        .route("/", get(|| async { "El servidor está activo." }))
        .route("/users", get(Cliente::get_many))
        .route("/err", get(error))
}
