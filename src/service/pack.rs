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
        paquete::{Paquete, PaqueteView},
    },
    traits::crud,
};

impl crud::Get for PaqueteView {
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

impl crud::Post for Paquete {
    type Input = Self;
    type Output = Self;
    async fn post_one(
        State(state): State<Arc<AppState>>,
        Json(payload): Json<Self>,
    ) -> Result<Json<Self>, ApiError> {
        let res = sqlx::query_as(
            "INSERT INTO paquete (nombre, descripcion, precio) VALUES ($1, $2, $3) RETURNING id, nombre, descripcion, precio",
        )
        .bind(payload.nombre)
        .bind(payload.descripcion)
        .bind(payload.precio)
        .fetch_one(&state.pg)
        .await?;

        Ok(Json(res))
    }
}

impl crud::Update for Paquete {
    async fn update_one(
        State(state): State<Arc<AppState>>,
        Path(id): Path<Uuid>,
        Json(payload): Json<Self>,
    ) -> Result<Json<Self>, ApiError> {
        if sqlx::query("SELECT * FROM paquete WHERE id=$1")
            .bind(id)
            .execute(&state.pg)
            .await?
            .rows_affected()
            == 0
        {
            return Err(ApiError::NotFound);
        }
        let res: Self = sqlx::query_as("UPDATE paquete SET nombre = $1, descripcion = $2, precio = $3 WHERE id = $4 RETURNING id, nombre, descripcion, precio")
            .bind(payload.nombre)
            .bind(payload.descripcion)
            .bind(payload.precio)
            .bind(id)
            .fetch_one(&state.pg)
            .await?;

        Ok(Json(res))
    }
}

impl crud::Delete for Paquete {
    async fn delete_one(
        State(state): State<Arc<AppState>>,
        Path(id): Path<Uuid>,
    ) -> Result<StatusCode, ApiError> {
        let res = sqlx::query("DELETE FROM paquete WHERE id = $1")
            .bind(id)
            .execute(&state.pg)
            .await?;
        if res.rows_affected() == 0 {
            Ok(StatusCode::NOT_FOUND)
        } else {
            Ok(StatusCode::NO_CONTENT)
        }
    }
}
