use axum::{Router, routing::get};

use crate::routes;

pub fn create_router() -> Router {
    Router::new()
        .route("/", get(|| async { "El servidor está activo." }))
        .merge(routes::user::user_routes())
}
