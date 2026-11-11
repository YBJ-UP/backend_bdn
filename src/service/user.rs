use axum::Json;

use crate::{crud, errors::auth_error::AuthError, model::usuario::Cliente};

impl crud::Get for Cliente {
    async fn get_many() -> Result<Json<Box<Self>>, crate::errors::auth_error::AuthError> {
        Err(AuthError::CreatedToken)
    }

    async fn get_one() -> Result<Json<Box<Self>>, AuthError> {
        Err(AuthError::InvalidToken)
    }
}
