use uuid::Uuid;

use crate::model::area::Department;

pub struct Office {
    id: Uuid,
    nombre: String,
    ubicacion: String,
    areas: Vec<Department>,
}
