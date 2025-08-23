use std::sync::Arc;

use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};

use crate::{
    handlers::todo::models::{PartialUpdateTodoRequest, Todo},
    service::{self, todo::Service},
};

pub async fn handler(
    State(todo_service): State<Arc<Service>>,
    Path(id): Path<u64>,
    Json(request): Json<PartialUpdateTodoRequest>,
) -> Result<StatusCode, StatusCode> {
    println!("Partial update TODO request: {request:?}");
    todo_service
        .partial_update(id as i32, request.into())
        .map_err(|error| {
            println!("Failed to do partial update: {error}");
            if matches!(
                error,
                service::todo::Error::Diesel(diesel::result::Error::NotFound)
            ) {
                return StatusCode::NOT_FOUND;
            }
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    Ok(StatusCode::OK)
}
