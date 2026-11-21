use std::sync::Arc;

use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};
use uuid::Uuid;

use crate::{
    AppState,
    model::{
        errors::ApiError,
        sucursal::{Office, OfficeView},
    },
    traits::crud,
};

impl crud::Get for OfficeView {
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

impl crud::Post for Office {
    type Input = Self;
    type Output = Self;
    async fn post_one(
        State(state): State<Arc<AppState>>,
        Json(payload): Json<Self>,
    ) -> Result<Json<Self>, ApiError> {
        let res = sqlx::query_as("INSERT INTO sucursal (nombre, ubicacion) VALUES ($1, $2) RETURNING id, nombre, ubicacion")
            .bind(payload.nombre)
            .bind(payload.ubicacion)
            .fetch_one(&state.pg)
            .await?;

        Ok(Json(res))
    }
}

impl crud::Update for Office {
    type Input = Self;
    type Output = Self;
    async fn update_one(
        State(state): State<Arc<AppState>>,
        Path(id): Path<Uuid>,
        Json(payload): Json<Self>,
    ) -> Result<Json<Self>, ApiError> {
        // Verificación de que la fila que se quiere buscar sí exista
        // Si no se hace esto retorna 500, cosa que no es intuitiva
        // No quiero tener que ponerle esto a las demás
        if sqlx::query("SELECT * FROM sucursal WHERE id=$1")
            .bind(id)
            .execute(&state.pg)
            .await?
            .rows_affected()
            == 0
        {
            return Err(ApiError::NotFound);
        }
        let res = sqlx::query_as(
            "UPDATE sucursal SET nombre = $1, ubicacion = $2 WHERE id = $3 RETURNING *",
        )
        .bind(payload.nombre)
        .bind(payload.ubicacion)
        .bind(id)
        .fetch_one(&state.pg)
        .await?;

        Ok(Json(res))
    }
}

impl crud::Delete for Office {
    async fn delete_one(
        State(state): State<Arc<AppState>>,
        Path(id): Path<Uuid>,
    ) -> Result<axum::http::StatusCode, ApiError> {
        let res = sqlx::query("DELETE FROM sucursal WHERE id=$1")
            .bind(id)
            .execute(&state.pg)
            .await?;

        if res.rows_affected() == 0 {
            Err(ApiError::NotFound)
        } else {
            Ok(StatusCode::NO_CONTENT)
        }
    }
}
