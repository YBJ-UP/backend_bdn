use axum::{Json, extract::Path};
use uuid::Uuid;

use crate::ApiError;

pub trait Get {
    fn get_many() -> impl Future<Output = Result<Json<Box<Self>>, ApiError>>;
    fn get_one(id: Path<Uuid>) -> impl Future<Output = Result<Json<Box<Self>>, ApiError>>;
}

pub trait Post {
    fn post_many() -> impl Future<Output = Result<Json<Box<Self>>, ApiError>>;
    fn post_one() -> impl Future<Output = Result<Json<Box<Self>>, ApiError>>;
}

pub trait Update {
    fn update_many() -> impl Future<Output = Result<Json<Box<Self>>, ApiError>>;
    fn update_one() -> impl Future<Output = Result<Json<Box<Self>>, ApiError>>;
}

pub trait Delete {
    fn delete_many() -> impl Future<Output = Result<Json<Box<Self>>, ApiError>>;
    fn delete_one() -> impl Future<Output = Result<Json<Box<Self>>, ApiError>>;
}
