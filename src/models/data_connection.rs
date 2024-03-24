use std::env::{self, VarError};

use sqlx::{postgres::PgPoolOptions, PgPool};

pub struct DatabaseConnection {}

impl DatabaseConnection {
    pub fn create() -> PgPool {
        let result: Result<String, VarError> = env::var("DATABASE_URL");
        let database_url = &result.unwrap();

        let connection = PgPoolOptions::new()
            .max_connections(16) // Set the maximum number of connections
            .connect_lazy(database_url)
            .expect("Failed to create pool");

        return connection;
    }
}
