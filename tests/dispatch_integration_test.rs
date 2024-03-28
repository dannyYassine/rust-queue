use rust_queue::{
    dispatch,
    models::{application::Application, job::JobStatus},
};

mod common;
use common::{set_up, PrintToConsoleJob};

#[tokio::test]
async fn it_should_add_job_to_table() {
    set_up();

    Application::bootstrap().await;
    let job_repository = JobRepository::new();
    job_repository.delete_all_jobs().await;

    let job = PrintToConsoleJob {
        name: "this is my job".to_string(),
    };

    dispatch!(job);

    let results = job_repository.get_all_jobs(Some(JobStatus::Pending)).await;

    let jobs = results.unwrap();
    assert_eq!(jobs.len(), 1);

    job_repository.delete_all_jobs().await;
}
