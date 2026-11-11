use std::sync::Arc;

use axum::{
    Json,
    extract::{Path, State},
};
use uuid::Uuid;

use crate::{
    ApiError, AppState, crud,
    model::usuario::{Cliente, Empleado},
};

impl crud::Get for Cliente {
    async fn get_many(State(state): State<Arc<AppState>>) -> Result<Json<Box<Self>>, ApiError> {
        Ok(Json(Box::new(Cliente::default())))
    }

    async fn get_one(
        State(state): State<Arc<AppState>>,
        Path(id): Path<Uuid>,
    ) -> Result<Json<Box<Self>>, ApiError> {
        Err(ApiError::InvalidToken)
    }
}

impl crud::Get for Empleado {
    async fn get_many(State(state): State<Arc<AppState>>) -> Result<Json<Box<Self>>, ApiError> {
        Err(ApiError::CreatedToken)
    }

    async fn get_one(
        State(state): State<Arc<AppState>>,
        Path(id): Path<Uuid>,
    ) -> Result<Json<Box<Self>>, ApiError> {
        Err(ApiError::CreatedToken)
    }
}
