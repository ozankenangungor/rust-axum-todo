use diesel::{
    Connection, ExpressionMethods, MysqlConnection, RunQueryDsl, SelectableHelper,
    query_dsl::methods::{FilterDsl, OrderDsl, SelectDsl},
    r2d2::{ConnectionManager, Pool},
};
use thiserror::Error;

use crate::db::{
    DbConnectionPoolError, connection_pool,
    models::{CreateTodo, TodoModel, UpdateTodo, UpdateTodoPartial},
    schema::{self, todos},
};

#[derive(Error, Debug)]
pub enum Error {
    #[error("Connection pool init error: {0}")]
    ConnectionPool(#[from] DbConnectionPoolError),
    #[error("R2D2 DB pool build error: {0}")]
    R2D2(#[from] r2d2::Error),
    #[error("DB error: {0}")]
    Diesel(#[from] diesel::result::Error),
}

pub struct Service {
    conn_pool: Pool<ConnectionManager<MysqlConnection>>,
}

impl Service {
    pub fn new() -> Result<Self, Error> {
        Ok(Self {
            conn_pool: connection_pool()?,
        })
    }

    pub fn create(&self, request: CreateTodo) -> Result<TodoModel, Error> {
        let mut conn = self.conn_pool.get()?;
        let result = conn.transaction(|conn| {
            diesel::insert_into(schema::todos::table)
                .values(&request)
                .execute(conn)?;

            schema::todos::table
                .order(schema::todos::id.desc())
                .select(TodoModel::as_select())
                .first(conn)
        })?;

        Ok(result)
    }

    pub fn list(&self) -> Result<Vec<TodoModel>, Error> {
        let mut conn = self.conn_pool.get()?;
        let result = schema::todos::table
            .select(TodoModel::as_select())
            .get_results(&mut conn)?;

        Ok(result)
    }

    pub fn get(&self, id: i32) -> Result<TodoModel, Error> {
        let mut conn = self.conn_pool.get()?;
        let result = schema::todos::table
            .select(TodoModel::as_select())
            .filter(schema::todos::id.eq(id))
            .first(&mut conn)?;

        Ok(result)
    }

    pub fn delete(&self, id: i32) -> Result<(), Error> {
        let mut conn = self.conn_pool.get()?;
        let result = diesel::delete(todos::table.filter(todos::id.eq(id))).execute(&mut conn)?;
        if result == 0 {
            return Err(Error::Diesel(diesel::result::Error::NotFound));
        }

        Ok(())
    }

    pub fn partial_update(&self, id: i32, request: UpdateTodoPartial) -> Result<(), Error> {
        let mut conn = self.conn_pool.get()?;
        let result = diesel::update(todos::table)
            .set(&request)
            .filter(todos::id.eq(id))
            .execute(&mut conn)?;
        if result == 0 {
            return Err(Error::Diesel(diesel::result::Error::NotFound));
        }

        Ok(())
    }

    pub fn update(&self, id: i32, request: UpdateTodo) -> Result<(), Error> {
        let mut conn = self.conn_pool.get()?;
        let result = diesel::update(todos::table)
            .set(&request)
            .filter(todos::id.eq(id))
            .execute(&mut conn)?;
        if result == 0 {
            return Err(Error::Diesel(diesel::result::Error::NotFound));
        }

        Ok(())
    }
}
