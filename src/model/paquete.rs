use uuid::Uuid;

use crate::model::servicio::Servicio;

struct Paquete {
    id: Uuid,
    nombre: String,
    descripcion: String,
    precio: f64,
    contenido: Vec<Servicio>,
}
