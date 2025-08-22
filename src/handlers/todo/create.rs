use std::sync::Arc;

use axum::{Json, extract::State, http::StatusCode};

use crate::{
    handlers::todo::models::{CreateTodoRequest, Todo},
    service::todo::Service,
};

pub async fn handler(
    State(todo_service): State<Arc<Service>>,
    Json(request): Json<CreateTodoRequest>,
) -> Result<Json<Todo>, StatusCode> {
    println!("Create TODO request: {request:?}");

    let result = todo_service.create(request.into()).map_err(|error| {
        println!("Error during creation of TODO: {}", error);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    Ok(Json(result.into()))
}
