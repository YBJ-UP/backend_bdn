use std::sync::Arc;

use axum::{Json, extract::State};

use crate::{
    AppState,
    model::{errors::ApiError, venta::Venta},
    traits::crud,
};

impl crud::Get for Venta {
    async fn get_many(
        state: axum::extract::State<std::sync::Arc<crate::AppState>>,
    ) -> Result<axum::Json<Vec<Self>>, ApiError> {
        let sales: Vec<Venta> = sqlx::query_as!(
            Venta,
            r#"SELECT id AS "id!", created_at AS "fecha!", estado AS "estado!", sucursal AS "sucursal!", cliente AS "cliente!", total AS "total!" FROM v_venta_resumen"#
        )
            .fetch_all(&state.pg)
            .await?;

        Ok(Json(sales))
    }

    async fn get_one(
        state: State<Arc<AppState>>,
        id: axum::extract::Path<uuid::Uuid>,
    ) -> Result<axum::Json<Self>, ApiError> {
        Err(ApiError::ExpiredToken)
    }
}
