use std::sync::Arc;

use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};
use serde::{Serialize, de::DeserializeOwned};
use uuid::Uuid;

use crate::{ApiError, AppState};

pub trait Filter: Sized {
    fn filter_many(
        state: State<Arc<AppState>>,
        id: Path<Uuid>,
    ) -> impl Future<Output = Result<Json<Vec<Self>>, ApiError>>;
}

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
    type Input: DeserializeOwned;
    type Output: Serialize;
    fn post_one(
        state: State<Arc<AppState>>,
        payload: Json<Self::Input>,
    ) -> impl Future<Output = Result<Json<Self::Output>, ApiError>>;
}

pub trait Update: Sized {
    type Input: DeserializeOwned;
    type Output: Serialize;
    fn update_one(
        state: State<Arc<AppState>>,
        id: Path<Uuid>,
        payload: Json<Self::Input>,
    ) -> impl Future<Output = Result<Json<Self::Output>, ApiError>>;
}

pub trait Delete: Sized {
    fn delete_one(
        state: State<Arc<AppState>>,
        id: Path<Uuid>,
    ) -> impl Future<Output = Result<StatusCode, ApiError>>;
}
