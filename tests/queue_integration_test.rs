#[cfg(test)]
mod tests {
    use dotenvy::dotenv;
    use rust_queue::{
        models::{
            job::{Job, JobStatus},
            queue::Queue,
        },
        repositories::job_repository::JobRepository,
    };

    #[tokio::test]
    async fn it_should_handle_job_in_database() {
        dotenv().ok();
        let job_repository = JobRepository::new().await;
        job_repository.delete_all_jobs().await;

        let job: Job = Job {
            id: 1,
            payload: "{}".to_string(),
            status: JobStatus::Pending.to_string(),
            model_type: "rust_queue::models::job::Job".to_string(),
            data: "".to_string(),
        };

        job_repository.add_job(&job).await;

        let results: Option<Vec<Job>> = job_repository.get_all_jobs(Some(JobStatus::Pending)).await;
        assert_eq!(results.unwrap().len(), 1);

        let mut queue: Queue = Queue::new_with_job_limit(1);
        queue.listen().await;

        let results: Option<Vec<Job>> = job_repository.get_all_jobs(Some(JobStatus::Pending)).await;
        assert_eq!(results.unwrap().len(), 0);

        job_repository.delete_all_jobs().await;
    }
}
