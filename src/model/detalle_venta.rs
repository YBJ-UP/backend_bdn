use bigdecimal::BigDecimal;
use serde::{Deserialize, Serialize};
use strum_macros::EnumString;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct DetalleVenta {
    pub id: Uuid,
    pub concepto: String,
    pub descripcion: String,
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
