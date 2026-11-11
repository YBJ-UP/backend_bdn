use axum::{Json, http::StatusCode, response::IntoResponse};
use serde_json::json;

pub enum AuthError {
    InvalidToken,
    MissingCredentials,
    ExpiredToken,
    WrongCredentials,
    CreatedToken,
}

impl IntoResponse for AuthError {
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
        };

        let body = Json(json!({ "error": msg }));

        (status, body).into_response()
    }
}
