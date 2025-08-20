use axum::{ extract::Path, http::StatusCode, Json };

use crate::handlers::todo::models::Todo;

pub async fn handler(Path(id): Path<u64>) -> (StatusCode, Json<Todo>) {
    println!("Get TODO handler id: {id}");
    (
        StatusCode::OK,
        Json(Todo {
            id: 0,
            title: "Some title".to_string(),
            description: "Some description".to_string(),
        }),
    )
}
