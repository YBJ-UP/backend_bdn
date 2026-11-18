use std::sync::Arc;

use axum::{
    Json,
    extract::{Path, State},
};
use uuid::Uuid;

use crate::{
    AppState,
    model::{errors::ApiError, movimientos},
    traits::crud,
};

impl crud::Filter for movimientos::MovimientoInterno {
    async fn filter_many(
        State(state): State<Arc<AppState>>,
        Path(id): Path<Uuid>,
    ) -> Result<Json<Vec<Self>>, ApiError> {
        println!("aa");
        let res: Vec<Self> = sqlx::query_as("SELECT id, created_at as fecha, sucursal, area, movimiento, tipo, costo, motivo FROM v_movimiento_detalle WHERE sucursal=$1")
            .bind(id)
            .fetch_all(&state.pg)
            .await?;

        Ok(Json(res))
    }
}
