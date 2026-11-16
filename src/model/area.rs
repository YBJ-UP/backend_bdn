use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use uuid::Uuid;

// debería de haber una vista que combine las áreas y la sucursal
// Se debe filtrar desde la base de datos para que solo saque los empleados de cada área
#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct Department {
    id: Uuid,
    nombre: String,
    supervisor: Option<String>,
    empleados: i64,
}
