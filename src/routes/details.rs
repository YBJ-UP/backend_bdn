use std::sync::Arc;

use axum::{Router, routing::get};

use crate::{AppState, model::detalle_venta::DetalleVenta, traits::crud::Filter};

pub fn details_routes() -> Router<Arc<AppState>> {
    Router::new().route("/details/{id}", get(DetalleVenta::filter_many))
}
