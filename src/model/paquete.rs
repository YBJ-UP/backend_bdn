use bigdecimal::BigDecimal;
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct PaqueteView {
    id: Option<Uuid>,
    pub nombre: String,
    pub descripcion: String,
    pub precio: BigDecimal,
    servicios: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Paquete {
    id: Option<Uuid>,
    pub nombre: String,
    pub descripcion: String,
    pub precio: BigDecimal,
}
