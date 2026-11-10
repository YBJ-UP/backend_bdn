use axum::{Router, routing::get};

use crate::{error, get_users};

pub fn create_router() -> Router {
    Router::new()
        .route("/health", get(|| async { "Es servidor está saludable" }))
        .route("/", get(|| async { "El servidor está activo." }))
        .route("/users", get(get_users))
        .route("/err", get(error))
}
