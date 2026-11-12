use std::sync::Arc;

use axum::{Router, routing::get};

use crate::{AppState, model::venta::Venta, traits::crud::Get};

pub fn sales_routes() -> Router<Arc<AppState>> {
    Router::new().route("/sales", get(Venta::get_many))
}
