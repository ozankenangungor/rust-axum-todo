use serde::{Deserialize, Serialize};

use crate::db::models::{CreateTodo, TodoModel, UpdateTodo, UpdateTodoPartial};

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

impl From<&TodoModel> for Todo {
    fn from(model: &TodoModel) -> Self {
        Self {
            id: model.id as u64,
            title: model.title.clone(),
            description: model.description.clone(),
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

impl From<PartialUpdateTodoRequest> for UpdateTodoPartial {
    fn from(value: PartialUpdateTodoRequest) -> Self {
        Self {
            title: value.title,
            description: value.description,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateTodoRequest {
    pub title: String,
    pub description: String,
}

impl From<UpdateTodoRequest> for UpdateTodo {
    fn from(value: UpdateTodoRequest) -> Self {
        Self {
            title: value.title,
            description: value.description,
        }
    }
}
