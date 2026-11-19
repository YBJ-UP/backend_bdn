use std::sync::Arc;

use axum::{
    Json,
    extract::{Path, State},
};
use uuid::Uuid;

use crate::{
    AppState,
    model::{errors::ApiError, servicio::Servicio},
    traits::crud,
};

impl crud::Filter for Servicio {
    async fn filter_many(
        State(state): State<Arc<AppState>>,
        Path(id): Path<Uuid>,
    ) -> Result<Json<Vec<Self>>, ApiError> {
        let res: Vec<Self> = sqlx::query_as(
            "SELECT id, nombre, descripcion, precio FROM v_contenido_paquete WHERE paquete=$1",
        )
        .bind(id)
        .fetch_all(&state.pg)
        .await?;

        Ok(Json(res))
    }
}

impl crud::Get for Servicio {
    async fn get_many(State(state): State<Arc<AppState>>) -> Result<Json<Vec<Self>>, ApiError> {
        let res: Vec<Self> = sqlx::query_as("SELECT id, nombre, descripcion, precio FROM servicio")
            .fetch_all(&state.pg)
            .await?;

        Ok(Json(res))
    }

    // NO implementado por el momento
    async fn get_one(
        _state: State<Arc<AppState>>,
        _id: Path<Uuid>,
    ) -> Result<Json<Self>, ApiError> {
        Err(ApiError::NotFound)
    }
}
