use axum::{ http::StatusCode, Json };

use crate::handlers::todo::models::{ CreateTodoRequest, Todo };

pub async fn handler(Json(request): Json<CreateTodoRequest>) -> (StatusCode, Json<Todo>) {
    println!("Create TODO request: {request:?}");
    (
        StatusCode::OK,
        Json(Todo {
            id: 0,
            title: "Some title".to_string(),
            description: "Some description".to_string(),
        }),
    )
}
