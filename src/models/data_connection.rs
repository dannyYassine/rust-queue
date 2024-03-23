use std::env::{self, VarError};

use sqlx::PgPool;

pub struct DatabaseConnection {}

impl DatabaseConnection {
    pub async fn create() -> PgPool {
        let result: Result<String, VarError> = env::var("DATABASE_URL");
        let connection = sqlx::PgPool::connect(&result.unwrap()).await.unwrap();

        return connection;
    }
}
