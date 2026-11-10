use uuid::Uuid;

use crate::model::user::User;

// debería de haber una vista que combine las áreas y la sucursal
// Se debe filtrar desde la base de datos para que solo saque los empleados de cada área
pub struct Department {
    id: Uuid,
    nombre: String,
    empleados: Vec<User>,
}
