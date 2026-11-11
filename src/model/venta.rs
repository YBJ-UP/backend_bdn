use uuid::Uuid;

use crate::model::{sucursal::Office, usuario::User};

enum EstadoVenta {
    Cancelado,
    Pendiente,
    Realizado,
}

enum ContenidoVenta {
    Servicio(Servicio),
    Paquete(Paquete),
}

struct Venta {
    id: Uuid,
    cliente: User,
    sucursal: Office,
    estado: EstadoVenta,
    contenido: Vec<ContenidoVenta>,
    fecha: String,
}

struct Servicio {
    id: Uuid,
    nombre: String,
    descripcion: String,
    precio: f64,
}

struct Paquete {
    id: Uuid,
    nombre: String,
    descripcion: String,
    precio: f64,
    contenido: Vec<Servicio>,
}
