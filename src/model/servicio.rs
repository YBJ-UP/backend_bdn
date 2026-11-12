use uuid::Uuid;

pub struct Servicio {
    id: Uuid,
    nombre: String,
    descripcion: String,
    precio: f64,
}
