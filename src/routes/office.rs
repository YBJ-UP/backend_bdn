use std::sync::Arc;

use axum::{Router, routing::get};

use crate::{AppState, model::sucursal::Office, traits::crud::Get};

pub fn office_routes() -> Router<Arc<AppState>> {
    Router::new()
        .route("/offices", get(Office::get_many))
        .route("/offices/{id}", get(Office::get_one))
}
