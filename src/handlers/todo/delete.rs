use axum::{ extract::Path, http::StatusCode };

pub async fn handler(Path(id): Path<u64>) -> StatusCode {
    println!("Delete TODO handler request id: {id}");
    StatusCode::OK
}
