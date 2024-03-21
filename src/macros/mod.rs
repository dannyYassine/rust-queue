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

        let job: Job = Job {
            id: 1,
            payload: "".to_string(),
            status: "pending".to_string(),
            model_type: "rust_queue::models::job::Job".to_string(),
            data: serde_json::to_string(&$job).unwrap()
        };

        let _ = sqlx::query(
            format!(
                "INSERT INTO jobs (payload, status, model_type, data) VALUES ('{}', '{}', '{}', '{}');",
                job.payload,
                job.status,
                job.model_type,
                job.data
            )
            .as_str(),
        )
        .execute(&connection)
        .await;
    };
}
