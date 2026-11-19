use std::sync::Arc;

use axum::{Router, routing::get};

use crate::{AppState, model::servicio::Servicio, traits::crud::Filter};

pub fn service_routes() -> Router<Arc<AppState>> {
    Router::new().route("/services/{id}", get(Servicio::filter_many))
}
