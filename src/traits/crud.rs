use axum::Json;

use crate::errors::auth_error::AuthError;

pub trait Get {
    fn get_many() -> Result<Json<Box<Self>>, AuthError>;
    fn get_one() -> Result<Json<Box<Self>>, AuthError>;
}

pub trait Post {
    fn post_many();
    fn post_one();
}

pub trait Update {
    fn update_many();
    fn update_one();
}

pub trait Delete {
    fn delete_many();
    fn delete_one();
}
