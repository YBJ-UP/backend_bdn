use std::sync::Arc;

use axum::{Router, routing::get};

use crate::{AppState, model::paquete::Paquete, traits::crud::Get};

pub fn pack_routes() -> Router<Arc<AppState>> {
    Router::new().route("/packs", get(Paquete::get_many))
}
