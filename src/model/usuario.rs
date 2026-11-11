use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use uuid::Uuid;

#[derive(Debug, Default, Serialize, Deserialize)]
enum Role {
    Admin,
    Ceo,
    Gerente,
    Supervisor,
    #[default]
    Empleado,
    Cliente,
}

#[derive(Debug, Default, Serialize, Deserialize)]
enum Status {
    Activo,
    #[default]
    Inactivo,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Empleado {
    id: Uuid,
    rol: Role,
    nombre: String,
    correo: String,
    telefono: Option<String>,
    salario: Option<f64>,
    estado: Status,
    id_sucursal: Option<Uuid>,
    id_area: Option<Uuid>,
}

#[derive(Debug, Default, Serialize, Deserialize, FromRow)]
pub struct Cliente {
    id: Uuid,
    nombre: String,
    correo: String,
    telefono: Option<String>,
}
