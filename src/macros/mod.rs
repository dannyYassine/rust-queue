#[macro_export]
macro_rules! dispatch {
    ($job:expr) => {
        use std::any::type_name_of_val;
        use std::env::{self, VarError};

        let result: Result<String, VarError> = env::var("DATABASE_URL");

        if result.is_err() {
            panic!("Missing DATABASE_URL");
        }
        let connection = sqlx::PgPool::connect(&result.unwrap()).await.unwrap();

        let _ = sqlx::query(
            format!(
                "INSERT INTO jobs (payload, status, model_type) VALUES ('{}', '{}', '{}');",
                $job.payload,
                $job.status,
                type_name_of_val(&$job)
            )
            .as_str(),
        )
        .execute(&connection)
        .await;
    };
}
