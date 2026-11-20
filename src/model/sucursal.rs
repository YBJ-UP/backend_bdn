use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use uuid::Uuid;

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct OfficeView {
    id: Uuid,
    nombre: String,
    ubicacion: String,
    areas: i64,
}

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct Office {
    id: Option<Uuid>,
    pub nombre: String,
    pub ubicacion: String,
}
