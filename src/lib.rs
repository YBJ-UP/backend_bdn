use axum::Json;

use crate::{
    errors::auth_error::AuthError,
    model::usuario::{Cliente, User},
    traits::crud,
};

pub mod errors {
    pub(crate) mod auth_error;
}

pub mod model {
    pub(crate) mod area;
    pub mod movimientos;
    pub mod sucursal;
    pub(crate) mod usuario;
    pub mod venta;
}

pub mod traits {
    pub mod crud;
}

pub mod service {
    pub mod user;
}

pub mod routes;

pub async fn get_users() -> Result<Json<User>, AuthError> {
    Ok(Json(User::default()))
}

pub async fn error() -> Result<Json<Cliente>, AuthError> {
    Err(AuthError::ExpiredToken)
}
