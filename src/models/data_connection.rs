use std::env::{self, VarError};

use sqlx::{postgres::PgPoolOptions, PgPool};

pub struct DatabaseConnection {}

impl DatabaseConnection {
    pub fn create() -> PgPool {
        let database_url = env::var("DATABASE_URL").unwrap();
        let database_pool = env::var("DATABASE_POOL").unwrap().parse::<u32>().unwrap();

        let connection = PgPoolOptions::new()
            .max_connections(database_pool) // Set the maximum number of connections
            .connect_lazy(&database_url)
            .expect("Failed to create pool");

        return connection;
    }
}
