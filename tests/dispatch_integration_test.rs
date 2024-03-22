use dotenvy::dotenv;
use rust_queue::{
    dispatch,
    models::job::{self, Job, JobStatus},
    repositories::job_repository,
};

mod common;
use common::PrintToConsoleJob;

#[tokio::test]
async fn it_should_add_job_to_table() {
    dotenv().ok();
    let job_repository = JobRepository::new().await;

    let job = PrintToConsoleJob {
        name: "this is my job".to_string(),
    };

    dispatch!(job);

    let results = job_repository.get_all_jobs(Some(JobStatus::Pending)).await;

    let jobs = results.unwrap();
    assert_eq!(jobs.len(), 1);

    job_repository.delete_all_jobs().await;
}
