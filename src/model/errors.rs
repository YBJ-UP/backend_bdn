use axum::{Json, http::StatusCode, response::IntoResponse};
use serde_json::json;
use sqlx::error::ErrorKind;

pub enum ApiError {
    InvalidToken,
    MissingCredentials,
    ExpiredToken,
    WrongCredentials,
    CreatedToken,
    NotFound,
    DbError(sqlx::Error),
}

impl From<sqlx::Error> for ApiError {
    fn from(e: sqlx::Error) -> Self {
        ApiError::DbError(e)
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> axum::response::Response {
        let (status, msg) = match self {
            Self::InvalidToken => (StatusCode::UNAUTHORIZED, "Credenciales inválidas"),
            Self::MissingCredentials => (StatusCode::UNAUTHORIZED, "Credenciales faltantes"),
            Self::ExpiredToken => (StatusCode::UNAUTHORIZED, "Credenciales expiradas"),
            Self::WrongCredentials => (
                StatusCode::FORBIDDEN,
                "No autorizado para realizar esta petición",
            ),
            Self::CreatedToken => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Error al generar el token",
            ),
            Self::NotFound => (StatusCode::NOT_FOUND, "No se encontró lo que se busca."),
            Self::DbError(e) => {
                tracing::error!("Error de base de datos: {}", e.to_string());
                db_err(&e)
            }
        };

        let body = Json(json!({ "error": msg }));

        (status, body).into_response()
    }
}

fn db_err(e: &sqlx::Error) -> (StatusCode, &'static str) {
    match e {
        sqlx::Error::RowNotFound => (StatusCode::NOT_FOUND, "No se encontró lo que se busca"),
        sqlx::Error::PoolClosed | sqlx::Error::PoolTimedOut | sqlx::Error::Io(_) => (
            StatusCode::SERVICE_UNAVAILABLE,
            "El servidor no se encuentra disponible, intente más tarde",
        ),
        sqlx::Error::Database(db) => match db.kind() {
            ErrorKind::UniqueViolation => (
                StatusCode::CONFLICT,
                "Ya existe un registro con estos datos",
            ),
            ErrorKind::CheckViolation | ErrorKind::NotNullViolation => {
                (StatusCode::UNPROCESSABLE_ENTITY, "Datos inválidos")
            }
            ErrorKind::ForeignKeyViolation => (
                StatusCode::UNPROCESSABLE_ENTITY,
                "Se está referenciando una entidad que no existe",
            ),
            _ => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Error en la base de datos",
            ),
        },
        _ => (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Error en la base de datos",
        ),
    }
}
