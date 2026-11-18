use std::sync::Arc;

use axum::{
    Json,
    extract::{Path, State},
};
use uuid::Uuid;

use crate::{
    AppState,
    model::{detalle_venta::DetalleVenta, errors::ApiError},
    traits::crud::Filter,
};

impl Filter for DetalleVenta {
    async fn filter_many(
        State(state): State<Arc<AppState>>,
        Path(id): Path<Uuid>,
    ) -> Result<Json<Vec<Self>>, ApiError> {
        let details: Vec<Self> = sqlx::query_as("SELECT id, concepto, descripcion, tipo, cantidad, precio, subtotal FROM v_detalle_venta WHERE id_venta=$1")
            .bind(id)
            .fetch_all(&state.pg)
            .await?;

        Ok(Json(details))
    }
}
