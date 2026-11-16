use std::sync::Arc;

use axum::{
    Json,
    extract::{Path, State},
};
use uuid::Uuid;

use crate::{
    AppState,
    model::{errors::ApiError, sucursal::Office},
    traits::crud,
};

impl crud::Get for Office {
    async fn get_many(State(state): State<Arc<AppState>>) -> Result<Json<Vec<Self>>, ApiError> {
        let res: Vec<Self> = sqlx::query_as("SELECT * FROM v_sucursal_area")
            .fetch_all(&state.pg)
            .await?;

        Ok(Json(res))
    }

    async fn get_one(
        State(state): State<Arc<AppState>>,
        Path(id): Path<Uuid>,
    ) -> Result<Json<Self>, ApiError> {
        let res: Self = sqlx::query_as("SELECT * FROM v_sucursal_area WHERE id = $1")
            .bind(id)
            .fetch_one(&state.pg)
            .await?;

        Ok(Json(res))
    }
}
