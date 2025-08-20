use axum::{ http::StatusCode, Json };

use crate::handlers::todo::models::{ UpdateTodoRequest, Todo };

pub async fn handler(Json(request): Json<UpdateTodoRequest>) -> (StatusCode, Json<Todo>) {
    println!("Update TODO request: {request:?}");
    (
        StatusCode::OK,
        Json(Todo {
            id: 0,
            title: "Some title".to_string(),
            description: "Some description".to_string(),
        }),
    )
}
