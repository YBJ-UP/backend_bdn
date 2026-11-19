use std::sync::Arc;

use axum::{
    Router,
    routing::{get, patch},
};

use crate::{AppState, model::paquete::Paquete, traits::crud::*};

pub fn pack_routes() -> Router<Arc<AppState>> {
    Router::new()
        .route("/packs", get(Paquete::get_many).post(Paquete::post_one))
        .route(
            "/packs/{id}",
            patch(Paquete::update_one).delete(Paquete::delete_one),
        )
}
