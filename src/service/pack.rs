use std::sync::Arc;

use axum::{
    Json,
    extract::{Path, State},
};

use crate::{
    AppState,
    model::{errors::ApiError, paquete::Paquete},
    traits::crud,
};

impl crud::Get for Paquete {
    async fn get_many(State(state): State<Arc<AppState>>) -> Result<Json<Vec<Self>>, ApiError> {
        let res: Vec<Self> = sqlx::query_as("SELECT * FROM v_paquete_servicio")
            .fetch_all(&state.pg)
            .await?;

        Ok(Json(res))
    }

    // No se implementa (o no veo razón para hacerlo)
    async fn get_one(
        State(_state): State<Arc<AppState>>,
        Path(_id): Path<uuid::Uuid>,
    ) -> Result<axum::Json<Self>, ApiError> {
        Err(ApiError::NotFound)
    }
}
