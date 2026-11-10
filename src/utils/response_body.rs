use axum::{Json, http::StatusCode, response::IntoResponse};
use serde_json::json;

pub fn to_response() {}

pub fn to_error(err: StatusCode, msg: String) -> axum::response::Response {
    let body = Json(json!({ "error": msg }));
    (err, body).into_response()
}
