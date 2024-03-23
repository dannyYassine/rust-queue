use rust_queue::{
    dispatch,
    models::{
        app_state::AppStateManager,
        job::{Job, JobStatus},
    },
};

mod common;
use common::{set_up, PrintToConsoleJob};

#[tokio::test]
async fn it_should_add_job_to_table() {
    set_up();

    let result: Result<String, VarError> = env::var("DATABASE_URL");
    let connection = sqlx::PgPool::connect(&result.unwrap()).await.unwrap();

    AppStateManager::get_instance()
        .initialize()
        .set_connection(connection);

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
