use diesel::{
    Selectable,
    data_types::MysqlTime,
    prelude::{AsChangeset, Insertable, Queryable},
};

use crate::handlers::todo::models::Todo;

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::db::schema::todos)]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
pub struct TodoModel {
    pub id: i32,
    pub title: String,
    pub description: String,
    pub created: MysqlTime,
    pub updated: MysqlTime,
}

#[derive(Insertable)]
#[diesel(table_name = crate::db::schema::todos)]
pub struct CreateTodo {
    pub title: String,
    pub description: String,
}

#[derive(AsChangeset)]
#[diesel(table_name = crate::db::schema::todos)]
pub struct UpdateTodoPartial {
    pub title: Option<String>,
    pub description: Option<String>,
}

#[derive(AsChangeset)]
#[diesel(table_name = crate::db::schema::todos)]
pub struct UpdateTodo {
    pub title: String,
    pub description: String,
}
