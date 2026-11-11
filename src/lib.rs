use sqlx::PgPool;

use crate::model::errors::ApiError;
use crate::traits::crud;

pub mod model {
    pub(crate) mod area;
    pub mod errors;
    pub mod movimientos;
    pub mod sucursal;
    pub(crate) mod usuario;
    pub mod venta;
}

pub mod traits {
    pub mod crud;
}

pub mod service {
    pub mod user;
}

pub mod routes {
    pub mod user;
}

pub struct AppState {
    pub pg: PgPool,
}

pub mod router;
