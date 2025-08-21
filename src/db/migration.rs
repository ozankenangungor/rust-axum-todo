use std::{ env };

use anyhow::{ anyhow, Ok };
use diesel::{ Connection, MysqlConnection };
use diesel_migrations::{ embed_migrations, EmbeddedMigrations, MigrationHarness };
use dotenvy::dotenv;

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations");

pub fn migrate_db() -> anyhow::Result<()> {
    let mut connection = db_connection()?;
    connection
        .run_pending_migrations(MIGRATIONS)
        .map_err(|error| anyhow!("Failed to run DB migrations: {error}"))?;

    Ok(())
}

fn db_connection() -> anyhow::Result<MysqlConnection> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")?;
    let connection = MysqlConnection::establish(&database_url)?;
    Ok(connection)
}
