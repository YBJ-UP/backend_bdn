use bigdecimal::BigDecimal;
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Paquete {
    id: Uuid,
    nombre: String,
    descripcion: String,
    precio: BigDecimal,
    servicios: i64,
}
