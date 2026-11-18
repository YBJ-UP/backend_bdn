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
enum TipoMovimiento {
    #[default]
    Compra,
    Venta,
}

impl From<String> for TipoMovimiento {
    fn from(value: String) -> Self {
        value.parse().unwrap_or_default()
    }
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct MovimientoInterno {
    id: Uuid,
    sucursal: Uuid,
    area: String,
    movimiento: String,
    #[sqlx(try_from = "String")]
    tipo: TipoMovimiento,
    costo: BigDecimal,
    motivo: String,
    #[serde(with = "fecha_fmt")]
    fecha: PlainDateTime,
}
