use std::sync::Arc;

use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};

use crate::{
    handlers::todo::models::Todo,
    service::{self, todo::Service},
};

pub async fn handler(
    State(todo_service): State<Arc<Service>>,
    Path(id): Path<u64>,
) -> Result<Json<Todo>, StatusCode> {
    println!("Get TODO handler id: {id}");

    let result = todo_service.get(id as i32).map_err(|error| {
        println!("Failed to retrieve TODO: {error}");
        if matches!(
            error,
            service::todo::Error::Diesel(diesel::result::Error::NotFound)
        ) {
            return StatusCode::NOT_FOUND;
        }
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    Ok(Json(result.into()))
}
