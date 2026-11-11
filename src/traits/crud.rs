use std::sync::Arc;

use axum::{
    Json,
    extract::{Path, State},
};
use uuid::Uuid;

use crate::{ApiError, AppState};

pub trait Get {
    fn get_many(
        state: State<Arc<AppState>>,
    ) -> impl Future<Output = Result<Json<Box<Self>>, ApiError>>;
    fn get_one(
        state: State<Arc<AppState>>,
        id: Path<Uuid>,
    ) -> impl Future<Output = Result<Json<Box<Self>>, ApiError>>;
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
