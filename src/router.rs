use std::sync::Arc;

use axum::{Router, routing::get};

use crate::{AppState, routes};

pub fn create_router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/", get(|| async { "El servidor está activo." }))
        .merge(routes::user::user_routes())
        .merge(routes::sale::sales_routes())
        .merge(routes::details::details_routes())
}
