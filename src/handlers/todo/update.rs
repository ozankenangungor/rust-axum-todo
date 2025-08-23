use std::sync::Arc;

use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};
use diesel::IntoSql;

use crate::{
    handlers::todo::models::{Todo, UpdateTodoRequest},
    service::{self, todo::Service},
};

pub async fn handler(
    State(todo_service): State<Arc<Service>>,
    Path(id): Path<u64>,
    Json(request): Json<UpdateTodoRequest>,
) -> Result<StatusCode, StatusCode> {
    println!("Update TODO request: {request:?}");

    let result = todo_service
        .update(id as i32, request.into())
        .map_err(|error| {
            println!("Failed to do update: {error}");
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
