use std::sync::Arc;

use axum::{Router, routing::get};

use crate::{AppState, model::area::Department, traits::crud::Filter};

pub fn department_routes() -> Router<Arc<AppState>> {
    Router::new().route("/departments/{id}", get(Department::filter_many))
}
