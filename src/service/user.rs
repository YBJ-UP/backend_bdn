use axum::{Json, extract::Path};
use uuid::Uuid;

use crate::{ApiError, crud, model::usuario::Cliente};

impl crud::Get for Cliente {
    async fn get_many() -> Result<Json<Box<Self>>, ApiError> {
        Ok(Json(Box::new(Cliente::default())))
    }

    async fn get_one(Path(id): Path<Uuid>) -> Result<Json<Box<Self>>, ApiError> {
        Err(ApiError::InvalidToken)
    }
}
