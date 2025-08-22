use std::sync::Arc;

use axum::{Json, extract::State, http::StatusCode};

use crate::{handlers::todo::models::Todo, service::todo::Service};

pub async fn handler(
    State(todo_service): State<Arc<Service>>,
) -> Result<Json<Vec<Todo>>, StatusCode> {
    let result = todo_service.list().map_err(|error| {
        println!("Failed to fetch the list of TODOs:  {}", error);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;
    Ok(Json(
        result
            .iter()
            .map(|value| {
                let result: Todo = value.into();
                result
            })
            .collect(),
    ))
}
