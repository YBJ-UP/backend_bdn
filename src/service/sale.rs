use std::sync::Arc;

use axum::{
    Json,
    extract::{Path, State},
};
use uuid::Uuid;

use crate::{
    AppState,
    model::{errors::ApiError, venta::Venta},
    traits::crud,
};

impl crud::Get for Venta {
    async fn get_many(
        State(state): State<Arc<crate::AppState>>,
    ) -> Result<Json<Vec<Self>>, ApiError> {
        let sales: Vec<Self> = sqlx::query_as!(
            Self,
            r#"SELECT id AS "id!", created_at AS "fecha!", estado AS "estado!", sucursal AS "sucursal!", cliente AS "cliente!", total AS "total!" FROM v_venta_resumen"#
        )
            .fetch_all(&state.pg)
            .await?;

        Ok(Json(sales))
    }

    async fn get_one(
        State(state): State<Arc<AppState>>,
        Path(id): Path<Uuid>,
    ) -> Result<Json<Self>, ApiError> {
        let sale: Self = sqlx::query_as!(
            Self,
            r#"SELECT id AS "id!", created_at AS "fecha!", estado AS "estado!", sucursal AS "sucursal!", cliente AS "cliente!", total AS "total!" FROM v_venta_resumen WHERE id=$1"#,
            id
        )
            .fetch_one(&state.pg)
            .await?;

        Ok(Json(sale))
    }
}
