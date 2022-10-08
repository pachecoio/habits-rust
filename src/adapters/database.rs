use diesel::sqlite::Sqlite;
use diesel::{sqlite::SqliteConnection};
use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;
use std::error::Error;

use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations");

fn run_migrations(connection: &mut impl MigrationHarness<Sqlite>) -> Result<(), Box<dyn Error + Send + Sync + 'static>> {

    // This will run the necessary migrations.
    //
    // See the documentation for `MigrationHarness` for
    // all available methods.
    connection.run_pending_migrations(MIGRATIONS)?;

    Ok(())
}

pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub fn establish_testing_connection() -> SqliteConnection {
    dotenv().ok();
    let mut connection = SqliteConnection::establish(":memory:").unwrap();
    run_migrations(&mut connection).unwrap();
    connection
}
