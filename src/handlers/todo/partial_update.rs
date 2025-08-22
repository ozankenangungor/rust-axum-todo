use axum::{Json, http::StatusCode};

use crate::handlers::todo::models::{PartialUpdateTodoRequest, Todo};

pub async fn handler(Json(request): Json<PartialUpdateTodoRequest>) -> (StatusCode, Json<Todo>) {
    println!("Partial update TODO request: {request:?}");
    (
        StatusCode::OK,
        Json(Todo {
            id: 0,
            title: "Some title".to_string(),
            description: "Some description".to_string(),
        }),
    )
}
