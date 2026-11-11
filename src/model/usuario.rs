use serde::{Deserialize, Serialize};
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

// esto o separarlos por rol aquí
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct User {
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

pub struct Empleado {
    id: Uuid,
}
