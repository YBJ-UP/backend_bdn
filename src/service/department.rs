use std::sync::Arc;

use axum::{
    Json,
    extract::{Path, State},
};
use uuid::Uuid;

use crate::{
    AppState,
    model::{area::Department, errors::ApiError},
    traits::crud,
};

impl crud::Filter for Department {
    async fn filter_many(
        State(state): State<Arc<AppState>>,
        Path(id): Path<Uuid>,
    ) -> Result<Json<Vec<Self>>, ApiError> {
        let res: Vec<Self> = sqlx::query_as(
            "SELECT id, nombre, supervisor, empleados FROM v_area_empleado WHERE sucursal = $1",
        )
        .bind(id)
        .fetch_all(&state.pg)
        .await?;

        Ok(Json(res))
    }
}
