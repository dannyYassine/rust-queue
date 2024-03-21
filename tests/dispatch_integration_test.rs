#[cfg(test)]
mod tests {
    use dotenvy::dotenv;
    use rust_queue::{
        dispatch,
        models::{custom_job::PrintToConsoleJob, job::Job},
    };

    #[tokio::test]
    async fn it_should_add_job_to_table() {
        dotenv().ok();
        let connection = sqlx::PgPool::connect(&env::var("DATABASE_URL").unwrap())
            .await
            .unwrap();
        let _ = sqlx::query("DELETE from jobs;").execute(&connection).await;

        let job = PrintToConsoleJob {
            name: "this is my job".to_string(),
        };

        dispatch!(job);

        let results: Result<Vec<Job>, _> = sqlx::query_as::<_, Job>(
            "SELECT id, payload, status, model_type, data FROM jobs where status = 'pending'",
        )
        .fetch_all(&connection)
        .await;

        let jobs = results.unwrap();
        assert_eq!(jobs.len(), 1);

        let _ = sqlx::query("DELETE from jobs;").execute(&connection).await;
    }
}
