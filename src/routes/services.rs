use std::sync::Arc;

use axum::{Router, routing::get};

use crate::{AppState, model::servicio::Servicio, traits::crud::*};

pub fn service_routes() -> Router<Arc<AppState>> {
    Router::new()
        .route(
            "/services/{id}",
            get(Servicio::filter_many)
                .patch(Servicio::update_one)
                .delete(Servicio::delete_one),
        )
        .route(
            "/services",
            get(Servicio::get_many).post(Servicio::post_one),
        )
}
