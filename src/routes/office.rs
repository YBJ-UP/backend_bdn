use std::sync::Arc;

use axum::{Router, routing::get};

use crate::{
    AppState,
    model::sucursal::{Office, OfficeView},
    traits::crud::*,
};

pub fn office_routes() -> Router<Arc<AppState>> {
    Router::new()
        .route("/offices", get(OfficeView::get_many).post(Office::post_one))
        .route(
            "/offices/{id}",
            get(OfficeView::get_one)
                .patch(Office::update_one)
                .delete(Office::delete_one),
        )
}
