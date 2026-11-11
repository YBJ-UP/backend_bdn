use std::sync::Arc;

use axum::{Router, routing::get};

use crate::{
    AppState,
    model::usuario::{Cliente, Empleado},
    traits::crud::Get,
};

pub fn user_routes() -> Router<Arc<AppState>> {
    Router::new()
        .route("/employees", get(Empleado::get_many))
        .route("/employees/{id}", get(Empleado::get_one))
        .route("/clients", get(Cliente::get_many))
        .route("/clientes/{id}", get(Cliente::get_one))
}
