use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use uuid::Uuid;

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct Office {
    id: Uuid,
    nombre: String,
    ubicacion: String,
    areas: i64,
}
