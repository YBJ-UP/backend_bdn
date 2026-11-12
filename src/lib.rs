use sqlx::PgPool;

use crate::model::errors::ApiError;
use crate::traits::crud;

pub mod model {
    pub(crate) mod area;
    pub mod errors;
    pub mod movimientos;
    pub mod sucursal;
    pub(crate) mod usuario;
    pub(crate) mod venta;
}

pub mod traits {
    pub mod crud;
}

pub mod service {
    pub mod client;
    pub mod employee;
    pub mod sale;
}

pub mod routes {
    pub mod sale;
    pub mod user;
}

pub struct AppState {
    pub pg: PgPool,
}

pub mod router;
