use bigdecimal::BigDecimal;
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use strum_macros::EnumString;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct DetalleVenta {
    pub id: Uuid,
    pub concepto: String,
    pub descripcion: String,
    #[sqlx(try_from = "String")]
    pub tipo: ContenidoVenta,
    pub cantidad: i32,
    pub precio: BigDecimal,
    pub subtotal: BigDecimal,
}

#[derive(Debug, Default, Serialize, Deserialize, EnumString)]
pub enum ContenidoVenta {
    #[default]
    Servicio,
    Paquete,
}

impl From<String> for ContenidoVenta {
    fn from(s: String) -> Self {
        s.parse().unwrap_or_default()
    }
}
