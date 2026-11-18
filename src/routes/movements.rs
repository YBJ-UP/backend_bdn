use std::sync::Arc;

use axum::{Router, routing::get};

use crate::{AppState, model::movimientos::MovimientoInterno, traits::crud::Filter};

pub fn movement_routes() -> Router<Arc<AppState>> {
    Router::new().route("/movements/{id}", get(MovimientoInterno::filter_many))
}
