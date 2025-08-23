use std::sync::Arc;

use axum::{
    extract::{Path, State},
    http::StatusCode,
};

use crate::service::{self, todo::Service};

pub async fn handler(
    State(todo_service): State<Arc<Service>>,
    Path(id): Path<u64>,
) -> Result<StatusCode, StatusCode> {
    println!("Delete TODO handler request id: {id}");

    todo_service.delete(id as i32).map_err(|error| {
        println!("Failed to delete TODO: {error}");
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
