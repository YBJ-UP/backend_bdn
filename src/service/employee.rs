use std::sync::Arc;

use axum::{
    Json,
    extract::{Path, State},
};
use uuid::Uuid;

use crate::{ApiError, AppState, crud, model::usuario::Empleado};

impl crud::Get for Empleado {
    async fn get_many(State(state): State<Arc<AppState>>) -> Result<Json<Vec<Self>>, ApiError> {
        let employees: Vec<Empleado> =
            sqlx::query_as!(Empleado,
                r#"SELECT id AS "id!", rol AS "rol!", nombre AS "nombre!", correo AS "correo!", telefono AS "telefono!", salario, estado AS "estado!", area, sucursal FROM v_usuario_detalle"#
                )
                .fetch_all(&state.pg)
                .await?;

        Ok(Json(employees))
    }

    async fn get_one(
        State(state): State<Arc<AppState>>,
        Path(id): Path<Uuid>,
    ) -> Result<Json<Self>, ApiError> {
        let employee: Option<Empleado> =
            sqlx::query_as!(Empleado,
                r#"SELECT id AS "id!", rol AS "rol!", nombre AS "nombre!", correo AS "correo!", telefono AS "telefono!", salario, estado AS "estado!", area, sucursal FROM v_usuario_detalle WHERE id=$1"#,
                id
                )
                .fetch_optional(&state.pg)
                .await?;

        match employee {
            None => Err(ApiError::NotFound),
            Some(e) => Ok(Json(e)),
        }
    }
}
