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
    async fn get_many(State(state): State<Arc<AppState>>) -> Result<Json<Vec<Self>>, ApiError> {
        let rows: Vec<Cliente> = sqlx::query_as("SELECT * FROM usuario WHERE id_rol=5")
            .fetch_all(&state.pg)
            .await?;

        Ok(Json(rows))
    }

    async fn get_one(
        State(state): State<Arc<AppState>>,
        Path(id): Path<Uuid>,
    ) -> Result<Json<Self>, ApiError> {
        let client: Option<Cliente> = sqlx::query_as!(
            Cliente,
            "SELECT id, nombre, correo, telefono FROM usuario WHERE id_rol=5 AND id = $1",
            id
        )
        .fetch_optional(&state.pg)
        .await?;

        match client {
            None => Err(ApiError::NotFound),
            Some(c) => Ok(Json(c)),
        }
    }
}

impl crud::Get for Empleado {
    async fn get_many(State(state): State<Arc<AppState>>) -> Result<Json<Vec<Self>>, ApiError> {
        Err(ApiError::CreatedToken)
    }

    async fn get_one(
        State(state): State<Arc<AppState>>,
        Path(id): Path<Uuid>,
    ) -> Result<Json<Self>, ApiError> {
        Err(ApiError::CreatedToken)
    }
}
