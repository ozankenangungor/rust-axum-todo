use std::env;
use diesel::{ r2d2::{ ConnectionManager, Pool }, MysqlConnection };
use dotenvy::dotenv;

pub mod migration;
pub mod schema;
pub mod models;

/// Getting a connection pool for database
pub fn connection_pool() -> anyhow::Result<Pool<ConnectionManager<MysqlConnection>>> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")?;
    let manager = ConnectionManager::<MysqlConnection>::new(database_url);

    Ok(Pool::builder().test_on_check_out(true).build(manager)?)
}
