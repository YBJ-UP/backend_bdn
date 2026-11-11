use axum::{Json, extract::Path};
use uuid::Uuid;

use crate::errors::auth_error::AuthError;

pub trait Get {
    fn get_many() -> impl Future<Output = Result<Json<Box<Self>>, AuthError>>;
    fn get_one(id: Path<Uuid>) -> impl Future<Output = Result<Json<Box<Self>>, AuthError>>;
}

pub trait Post {
    fn post_many() -> impl Future<Output = Result<Json<Box<Self>>, AuthError>>;
    fn post_one() -> impl Future<Output = Result<Json<Box<Self>>, AuthError>>;
}

pub trait Update {
    fn update_many() -> impl Future<Output = Result<Json<Box<Self>>, AuthError>>;
    fn update_one() -> impl Future<Output = Result<Json<Box<Self>>, AuthError>>;
}

pub trait Delete {
    fn delete_many() -> impl Future<Output = Result<Json<Box<Self>>, AuthError>>;
    fn delete_one() -> impl Future<Output = Result<Json<Box<Self>>, AuthError>>;
}
