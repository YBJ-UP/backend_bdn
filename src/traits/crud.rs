use std::sync::Arc;

use axum::{
    Json,
    extract::{Path, State},
};
use uuid::Uuid;

use crate::{ApiError, AppState};

pub trait Get: Sized {
    fn get_many(
        state: State<Arc<AppState>>,
    ) -> impl Future<Output = Result<Json<Vec<Self>>, ApiError>>;
    fn get_one(
        state: State<Arc<AppState>>,
        id: Path<Uuid>,
    ) -> impl Future<Output = Result<Json<Self>, ApiError>>;
}

pub trait Post: Sized {
    fn post_one(
        state: State<Arc<AppState>>,
        payload: Json<Self>,
    ) -> impl Future<Output = Result<Json<Self>, ApiError>>;
}

pub trait Update: Sized {
    fn update_one(
        state: State<Arc<AppState>>,
        id: Path<Uuid>,
        payload: Json<Self>,
    ) -> impl Future<Output = Result<Json<Self>, ApiError>>;
}

pub trait Delete: Sized {
    fn delete_one(
        state: State<Arc<AppState>>,
        id: Path<Uuid>,
        payload: Json<Self>,
    ) -> impl Future<Output = Result<Json<Self>, ApiError>>;
}
