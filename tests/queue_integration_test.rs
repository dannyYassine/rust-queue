#[cfg(test)]
mod tests {
    use dotenvy::dotenv;
    use rust_queue::models::{
        job::{Job, JobStatus},
        queue::Queue,
    };
    use std::env;

    #[tokio::test]
    async fn it_should_handle_job_in_database() {
        dotenv().ok();
        let job: Job = Job {
            id: 1,
            payload: "{}".to_string(),
            status: JobStatus::Pending.to_string(),
            model_type: "rust_queue::models::job::Job".to_string(),
        };
        let connection = sqlx::PgPool::connect(&env::var("DATABASE_URL").unwrap())
            .await
            .unwrap();
        let _ = sqlx::query(
            format!(
                "INSERT INTO jobs (payload, status, model_type) VALUES ('{}', '{}', '{}');",
                job.payload, job.status, job.model_type
            )
            .as_str(),
        )
        .execute(&connection)
        .await;

        let results: Result<Vec<Job>, _> = sqlx::query_as::<_, Job>(
            "SELECT id, payload, status, model_type FROM jobs where status = 'pending'",
        )
        .fetch_all(&connection)
        .await;

        assert_eq!(results.unwrap().len(), 1);

        let mut queue: Queue = Queue::new_with_job_limit(1);
        queue.listen().await;

        let results: Result<Vec<Job>, _> = sqlx::query_as::<_, Job>(
            "SELECT id, payload, status, model_type FROM jobs where status = 'pending'",
        )
        .fetch_all(&connection)
        .await;

        assert_eq!(results.unwrap().len(), 0);

        let _ = sqlx::query("DELETE from jobs;").execute(&connection).await;
    }
}
