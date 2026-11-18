use sqlx::PgPool;

use crate::model::errors::ApiError;
use crate::traits::crud;

pub mod model {
    pub mod area;
    pub mod detalle_venta;
    pub mod errors;
    pub mod movimientos;
    pub mod paquete;
    pub mod servicio;
    pub mod sucursal;
    pub mod usuario;
    pub mod venta;
}

pub mod traits {
    pub mod crud;
}

pub mod service {
    pub mod client;
    pub mod department;
    pub mod employee;
    pub mod movement;
    pub mod office;
    pub mod pack;
    pub mod sale;
    pub mod sale_details;
}

pub mod routes {
    pub mod department;
    pub mod details;
    pub mod movements;
    pub mod office;
    pub mod packs;
    pub mod sale;
    pub mod user;
}

pub struct AppState {
    pub pg: PgPool,
}

pub mod router;
