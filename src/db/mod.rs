use std::env::{ self, VarError };
use diesel::{ r2d2::{ ConnectionManager, Pool }, MysqlConnection };
use dotenvy::dotenv;
use thiserror::Error;

pub mod migration;
pub mod schema;
pub mod models;

#[derive(Error, Debug)]
pub enum DbConnectionPoolError {
    #[error("Missing environment variable: {0}")] EnvVar(#[from] VarError),
    #[error("R2D2 DB pool build error: {0}")] R2D2(#[from] r2d2::Error),
}

/// Getting a connection pool for database
pub fn connection_pool() -> Result<
    Pool<ConnectionManager<MysqlConnection>>,
    DbConnectionPoolError
> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")?;
    let manager = ConnectionManager::<MysqlConnection>::new(database_url);

    Ok(Pool::builder().test_on_check_out(true).build(manager)?)
}
