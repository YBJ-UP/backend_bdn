use axum::{Router, routing::get};

use crate::{model::usuario::Cliente, traits::crud::Get};

pub fn create_router() -> Router {
    Router::new()
        .route("/health", get(|| async { "Es servidor está saludable" }))
        .route("/", get(|| async { "El servidor está activo." }))
        .route("/users", get(Cliente::get_many))
        .route("/user/{id}", get(Cliente::get_one))
}
