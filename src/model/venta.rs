use bigdecimal::BigDecimal;
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use strum_macros::EnumString;
use time::{PlainDateTime, serde::format_description};
use uuid::Uuid;

format_description!(
    fecha_fmt,
    PlainDateTime,
    "[year]-[month]-[day] [hour]:[minute]:[second]"
);

#[derive(Debug, Default, Serialize, Deserialize, EnumString)]
#[strum(ascii_case_insensitive)]
pub enum EstadoVenta {
    Cancelado,
    #[default]
    Pendiente,
    Realizado,
}

impl From<String> for EstadoVenta {
    fn from(s: String) -> Self {
        s.parse().unwrap_or_default()
    }
}

enum ContenidoVenta {
    Servicio(Servicio),
    Paquete(Paquete),
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Venta {
    pub id: Uuid,
    pub cliente: String,
    pub sucursal: String,
    pub estado: EstadoVenta,
    #[serde(with = "fecha_fmt")]
    pub fecha: PlainDateTime,
    pub total: BigDecimal,
}

struct Servicio {
    id: Uuid,
    nombre: String,
    descripcion: String,
    precio: f64,
}

struct Paquete {
    id: Uuid,
    nombre: String,
    descripcion: String,
    precio: f64,
    contenido: Vec<Servicio>,
}
