use crate::errors::auth_error::AuthError;
use crate::traits::crud;

pub mod errors {
    pub(crate) mod auth_error;
}

pub mod model {
    pub(crate) mod area;
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

pub mod routes;
