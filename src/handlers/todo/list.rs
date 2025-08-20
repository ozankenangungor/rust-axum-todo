use axum::{ http::StatusCode, Json };

use crate::handlers::todo::models::Todo;

pub async fn handler() -> (StatusCode, Json<Vec<Todo>>) {
    (
        StatusCode::OK,
        Json(
            vec![Todo {
                id: 0,
                title: "Some title".to_string(),
                description: "Some description".to_string(),
            }]
        ),
    )
}
