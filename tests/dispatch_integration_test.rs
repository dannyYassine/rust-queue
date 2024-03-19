#[cfg(test)]
mod tests {
    use dotenvy::dotenv;
    use rust_queue::{
        dispatch,
        models::job::{Job, JobStatus},
    };

    #[tokio::test]
    async fn it_should_add_job_to_table() {
        dotenv().ok();
        let job: Job = Job {
            id: 1,
            payload: "{}".to_string(),
            status: JobStatus::Pending.to_string(),
            model_type: "rust_queue::models::job::Job".to_string(),
        };

        dispatch!(job);

        let connection = sqlx::PgPool::connect(&env::var("DATABASE_URL").unwrap())
            .await
            .unwrap();

        let results: Result<Vec<Job>, _> = sqlx::query_as::<_, Job>(
            "SELECT id, payload, status, model_type FROM jobs where status = 'pending'",
        )
        .fetch_all(&connection)
        .await;

        let jobs = results.unwrap();
        assert_eq!(jobs.len(), 1);

        let job_in_database = jobs.first().unwrap();

        assert_eq!(job.payload, job_in_database.payload);
        assert_eq!(job.model_type, job_in_database.model_type);
        assert_eq!(job.status, job_in_database.status);

        sqlx::query("DELETE from jobs;".as_str())
            .execute(&connection)
            .await;
    }
}
