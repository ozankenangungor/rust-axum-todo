use std::sync::Arc;

use axum::{ routing::{ delete, get, patch, post, put }, Router };

use crate::db::{ migration::migrate_db };

mod handlers;
mod db;
mod service;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    migrate_db()?;

    let todo_service = Arc::new(service::todo::Service::new()?);

    let router = Router::new()
        .route("/todo", get(handlers::todo::list::handler))
        .route("/todo/{id}", get(handlers::todo::get::handler))
        .route("/todo", post(handlers::todo::create::handler))
        .route("/todo/{id}", delete(handlers::todo::delete::handler))
        .route("/todo/{id}", patch(handlers::todo::partial_update::handler))
        .route("/todo/{id}", put(handlers::todo::update::handler))
        .with_state(todo_service);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:9999").await?;
    axum::serve(listener, router).await?;
    Ok(())
}
