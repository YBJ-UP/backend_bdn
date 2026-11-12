use axum::{Json, http::StatusCode, response::IntoResponse};
use serde_json::json;

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
                tracing::error!("Error de base de datos: {e}");
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Error en la base de datos",
                )
            }
        };

        let body = Json(json!({ "error": msg }));

        (status, body).into_response()
    }
}
