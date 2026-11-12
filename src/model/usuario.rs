use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use strum_macros::EnumString;
use uuid::Uuid;

#[derive(Debug, Default, Serialize, Deserialize, EnumString)]
pub enum Role {
    #[strum(ascii_case_insensitive)]
    Admin,
    #[strum(ascii_case_insensitive)]
    Ceo,
    #[strum(ascii_case_insensitive)]
    Gerente,
    #[strum(ascii_case_insensitive)]
    Supervisor,
    #[strum(ascii_case_insensitive)]
    #[default]
    Empleado,
    #[strum(ascii_case_insensitive)]
    Cliente,
}

#[derive(Debug, Default, Serialize, Deserialize, EnumString)]
pub enum Status {
    #[strum(ascii_case_insensitive)]
    Activo,
    #[strum(ascii_case_insensitive)]
    #[default]
    Inactivo,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Empleado {
    pub id: Uuid,
    pub rol: Role,
    pub nombre: String,
    pub correo: String,
    pub telefono: String,
    pub salario: f32,
    pub estado: Status,
    pub sucursal: String,
    pub area: String,
}

#[derive(Debug, Default, Serialize, Deserialize, FromRow)]
pub struct Cliente {
    pub id: Option<Uuid>,
    pub nombre: String,
    pub correo: String,
    pub telefono: Option<String>,
}
