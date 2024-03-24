use rust_queue::{
    models::{
        application::Application,
        job::{Job, JobStatus},
        queue::Queue,
    },
    repositories::job_repository::JobRepository,
};

mod common;
use common::set_up;

#[tokio::test]
async fn it_should_handle_job_in_database() {
    set_up();

    Application::bootstrap().await;

    let job_repository = JobRepository::new();
    job_repository.delete_all_jobs().await;

    let job: Job = Job::new(
        "{}".to_string(),
        JobStatus::Pending.to_string(),
        "rust_queue::models::job::Job".to_string(),
        "".to_string(),
    );

    job_repository.add_job(&job).await;

    let results: Option<Vec<Job>> = job_repository.get_all_jobs(Some(JobStatus::Pending)).await;
    assert_eq!(results.unwrap().len(), 1);

    let mut queue: Queue = Queue::new_with_job_limit(1);
    queue.listen().await;

    let results: Option<Vec<Job>> = job_repository.get_all_jobs(Some(JobStatus::Pending)).await;
    assert_eq!(results.unwrap().len(), 0);

    job_repository.delete_all_jobs().await;
}
