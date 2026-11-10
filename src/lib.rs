use axum::Json;

use crate::{errors::auth_error::AuthError, model::user::User};

pub mod errors {
    pub(crate) mod auth_error;
}

pub mod utils {
    pub(crate) mod response_body;
}

pub mod model {
    pub(crate) mod area;
    pub mod sucursal;
    pub(crate) mod user;
}

pub mod routes;

pub async fn get_users() -> Result<Json<User>, AuthError> {
    Ok(Json(User::default()))
}

pub async fn error() -> Result<Json<User>, AuthError> {
    Err(AuthError::ExpiredToken)
}
