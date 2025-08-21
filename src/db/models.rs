use diesel::{ data_types::MysqlTime, prelude::Queryable, Selectable };
#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::db::schema::todos)]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
pub struct Todo {
    pub id: i32,
    pub title: String,
    pub description: String,
    pub created: MysqlTime,
    pub updated: MysqlTime,
}
