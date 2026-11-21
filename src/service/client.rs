use std::sync::Arc;

use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};
use uuid::Uuid;

use crate::{ApiError, AppState, crud, model::usuario::Cliente};

impl crud::Get for Cliente {
    async fn get_many(State(state): State<Arc<AppState>>) -> Result<Json<Vec<Self>>, ApiError> {
        let rows: Vec<Self> = sqlx::query_as("SELECT id, nombre, correo, telefono FROM v_cliente")
            .fetch_all(&state.pg)
            .await?;

        Ok(Json(rows))
    }

    async fn get_one(
        State(state): State<Arc<AppState>>,
        Path(id): Path<Uuid>,
    ) -> Result<Json<Self>, ApiError> {
        let client: Option<Self> =
            sqlx::query_as("SELECT id, nombre, correo, telefono FROM v_cliente WHERE id = $1")
                .bind(id)
                .fetch_optional(&state.pg)
                .await?;

        match client {
            None => Err(ApiError::NotFound),
            Some(c) => Ok(Json(c)),
        }
    }
}

impl crud::Post for Cliente {
    type Input = Self;
    type Output = Self;
    async fn post_one(
        State(state): State<Arc<AppState>>,
        Json(payload): Json<Self::Input>,
    ) -> Result<Json<Self::Output>, ApiError> {
        let res: Self = sqlx::query_as(
            "INSERT INTO usuario (nombre, correo, telefono, id_rol) VALUES ($1, $2, $3, 6) RETURNING id, nombre, correo, telefono",
        )
        .bind(payload.nombre)
        .bind(payload.correo)
        .bind(payload.telefono)
        .fetch_one(&state.pg)
        .await?;

        Ok(Json(res))
    }
}

impl crud::Update for Cliente {
    type Input = Self;
    type Output = Self;
    async fn update_one(
        State(state): State<Arc<AppState>>,
        Path(id): Path<Uuid>,
        Json(payload): Json<Self>,
    ) -> Result<Json<Self>, ApiError> {
        let res: Self = sqlx::query_as("UPDATE usuario SET nombre=$1, correo=$2, telefono=$3 WHERE id=$4 RETURNING id, nombre, correo, telefono")
                .bind(payload.nombre)
                .bind(payload.correo)
                .bind(payload.telefono)
                .bind(id)
                .fetch_one(&state.pg)
                .await?;

        Ok(Json(res))
    }
}

impl crud::Delete for Cliente {
    async fn delete_one(
        State(state): State<Arc<AppState>>,
        Path(id): Path<Uuid>,
    ) -> Result<axum::http::StatusCode, ApiError> {
        let res = sqlx::query("DELETE FROM usuario WHERE id=$1")
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
