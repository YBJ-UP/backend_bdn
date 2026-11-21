use std::sync::Arc;

use axum::{
    Json,
    extract::{Path, State},
};
use uuid::Uuid;

use crate::{
    ApiError, AppState, crud,
    model::usuario::{Empleado, EmpleadoPost},
    utils::hash::hash_password,
};

impl crud::Get for Empleado {
    async fn get_many(State(state): State<Arc<AppState>>) -> Result<Json<Vec<Self>>, ApiError> {
        let employees: Vec<Empleado> =
            sqlx::query_as( "SELECT id, rol, nombre, correo, telefono, salario, estado, area, sucursal FROM v_usuario_detalle")
                .fetch_all(&state.pg)
                .await?;

        Ok(Json(employees))
    }

    async fn get_one(
        State(state): State<Arc<AppState>>,
        Path(id): Path<Uuid>,
    ) -> Result<Json<Self>, ApiError> {
        let employee: Option<Empleado> =
            sqlx::query_as("SELECT id, rol, nombre, correo, telefono, salario, estado, area, sucursal FROM v_usuario_detalle WHERE id=$1")
            .bind(id)
                .fetch_optional(&state.pg)
                .await?;

        match employee {
            None => Err(ApiError::NotFound),
            Some(e) => Ok(Json(e)),
        }
    }
}

impl crud::Post for EmpleadoPost {
    async fn post_one(
        State(state): State<Arc<AppState>>,
        Json(payload): Json<Self::Input>,
    ) -> Result<Json<Self::Output>, ApiError> {
        let pw_hash = hash_password(&payload.contrasenna)?;
        tracing::info!("Hash generado");
        let created: Uuid = sqlx::query_scalar(
            "INSERT INTO usuario
                (id_rol, nombre, correo, telefono, contrasena, salario, id_sucursal, id_area, supervisor, estado)
            VALUES
                ((SELECT id FROM rol WHERE nombre=$1), $2, $3, $4, $5, $6, $7, $8, $9, $10)
            RETURNING id"
            )
            .bind(payload.rol.to_string())
            .bind(payload.nombre)
            .bind(payload.correo)
            .bind(payload.telefono)
            .bind(pw_hash)
            .bind(payload.salario)
            .bind(payload.sucursal)
            .bind(payload.area)
            .bind(payload.supervisor)
            .bind(payload.estado.unwrap_or_default().to_string())
            .fetch_one(&state.pg)
            .await?;

        let res: Self::Output = sqlx::query_as(
            "SELECT id, rol, nombre, correo, telefono, salario, estado, area, sucursal
            FROM v_usuario_detalle WHERE id=$1",
        )
        .bind(created)
        .fetch_one(&state.pg)
        .await?;

        Ok(Json(res))
    }

    type Input = EmpleadoPost;

    type Output = Empleado;
}
