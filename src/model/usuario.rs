use serde::{Deserialize, Serialize};
use sqlx::{prelude::FromRow, types::BigDecimal};
use strum_macros::EnumString;
use uuid::Uuid;

#[derive(Debug, Default, Serialize, Deserialize, EnumString)]
#[strum(ascii_case_insensitive)]
pub enum Role {
    Admin,
    Ceo,
    Gerente,
    Supervisor,
    #[default]
    Empleado,
    Cliente,
}

#[derive(Debug, Default, Serialize, Deserialize, EnumString)]
#[strum(ascii_case_insensitive)]
pub enum Status {
    #[default]
    Activo,
    Baja,
    Pausado,
}

impl From<String> for Role {
    fn from(s: String) -> Self {
        s.parse().unwrap_or_default()
    }
}

impl From<String> for Status {
    fn from(s: String) -> Self {
        s.parse().unwrap_or_default()
    }
}

#[derive(Debug, Default, Serialize, Deserialize, FromRow)]
pub struct Empleado {
    pub id: Uuid,
    #[sqlx(try_from = "String")]
    pub rol: Role,
    pub nombre: String,
    pub correo: String,
    pub telefono: String,
    pub salario: Option<BigDecimal>,
    #[sqlx(try_from = "String")]
    pub estado: Status,
    pub sucursal: Option<String>,
    pub area: Option<String>,
}

#[derive(Debug, Default, Serialize, Deserialize, FromRow)]
pub struct Cliente {
    pub id: Option<Uuid>,
    pub nombre: String,
    pub correo: String,
    pub telefono: Option<String>,
}
