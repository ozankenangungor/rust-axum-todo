use serde::{ Deserialize, Serialize };

use crate::db::models::{ CreateTodo, TodoModel };

#[derive(Debug, Serialize, Deserialize)]
pub struct Todo {
    pub id: u64,
    pub title: String,
    pub description: String,
}

impl From<TodoModel> for Todo {
    fn from(model: TodoModel) -> Self {
        Self {
            id: model.id as u64,
            title: model.title,
            description: model.description,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateTodoRequest {
    pub title: String,
    pub description: String,
}

impl From<CreateTodoRequest> for CreateTodo {
    fn from(value: CreateTodoRequest) -> Self {
        Self {
            title: value.title,
            description: value.description,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PartialUpdateTodoRequest {
    pub title: Option<String>,
    pub description: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateTodoRequest {
    pub title: String,
    pub description: String,
}
