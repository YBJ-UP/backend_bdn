use crate::traits::crud::Get;
use axum::Json;

use crate::{errors::auth_error::AuthError, model::usuario::User, traits::crud};

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

pub mod routes;

pub async fn get_users() -> Result<Json<User>, AuthError> {
    Ok(Json(User::default()))
}

pub async fn error() -> Result<Json<User>, AuthError> {
    Err(AuthError::ExpiredToken)
}

impl crud::Get for User {
    fn get_one() -> Result<Json<Box<User>>, AuthError> {
        Err(AuthError::ExpiredToken)
    }

    fn get_many() -> Result<Json<Box<Self>>, AuthError> {
        Err(AuthError::MissingCredentials)
    }
}

fn lol() {
    let res = User::get_one();
    println!("{:?}", res.ok())
}
