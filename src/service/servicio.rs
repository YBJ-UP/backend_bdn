use std::sync::Arc;

use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};
use uuid::Uuid;

use crate::{
    AppState,
    model::{errors::ApiError, servicio::Servicio},
    traits::crud,
};

impl crud::Filter for Servicio {
    async fn filter_many(
        State(state): State<Arc<AppState>>,
        Path(id): Path<Uuid>,
    ) -> Result<Json<Vec<Self>>, ApiError> {
        let res: Vec<Self> = sqlx::query_as(
            "SELECT id, nombre, descripcion, precio FROM v_contenido_paquete WHERE paquete=$1",
        )
        .bind(id)
        .fetch_all(&state.pg)
        .await?;

        Ok(Json(res))
    }
}

impl crud::Get for Servicio {
    async fn get_many(State(state): State<Arc<AppState>>) -> Result<Json<Vec<Self>>, ApiError> {
        let res: Vec<Self> = sqlx::query_as("SELECT id, nombre, descripcion, precio FROM servicio")
            .fetch_all(&state.pg)
            .await?;

        Ok(Json(res))
    }

    // NO implementado por el momento
    async fn get_one(
        _state: State<Arc<AppState>>,
        _id: Path<Uuid>,
    ) -> Result<Json<Self>, ApiError> {
        Err(ApiError::NotFound)
    }
}

impl crud::Post for Servicio {
    async fn post_one(
        State(state): State<Arc<AppState>>,
        Json(payload): Json<Self>,
    ) -> Result<Json<Self>, ApiError> {
        let res: Self = sqlx::query_as(
            "INSERT INTO servicio (nombre, descripcion, precio) VALUES ($1, $2, $3) RETURNING *",
        )
        .bind(payload.nombre)
        .bind(payload.descripcion)
        .bind(payload.precio)
        .fetch_one(&state.pg)
        .await?;

        Ok(Json(res))
    }
}

impl crud::Update for Servicio {
    async fn update_one(
        State(state): State<Arc<AppState>>,
        Path(id): Path<Uuid>,
        Json(payload): Json<Self>,
    ) -> Result<Json<Self>, ApiError> {
        if sqlx::query("SELECT * FROM servicio WHERE id=$1")
            .bind(id)
            .execute(&state.pg)
            .await?
            .rows_affected()
            == 0
        {
            return Err(ApiError::NotFound);
        }
        let res: Self = sqlx::query_as("UPDATE servicio SET nombre = $1, descripcion = $2, precio = $3 WHERE id = $4 RETURNING *")
            .bind(payload.nombre)
            .bind(payload.descripcion)
            .bind(payload.precio)
            .bind(id)
            .fetch_one(&state.pg)
            .await?;

        Ok(Json(res))
    }
}

impl crud::Delete for Servicio {
    async fn delete_one(
        State(state): State<Arc<AppState>>,
        Path(id): Path<Uuid>,
    ) -> Result<StatusCode, ApiError> {
        let res = sqlx::query("DELETE FROM servicio WHERE id = $1")
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
