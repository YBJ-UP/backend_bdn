use std::sync::Arc;

use axum::{
    Router,
    routing::{get, patch},
};

use crate::{
    AppState,
    model::paquete::{Paquete, PaqueteView},
    traits::crud::*,
};

pub fn pack_routes() -> Router<Arc<AppState>> {
    Router::new()
        .route("/packs", get(PaqueteView::get_many).post(Paquete::post_one))
        .route(
            "/packs/{id}",
            patch(Paquete::update_one).delete(Paquete::delete_one),
        )
}
