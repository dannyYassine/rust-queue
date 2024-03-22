#[cfg(test)]
mod tests {
    use rust_queue::{
        dispatch,
        models::job::{Job, JobStatus},
    };
    use crate::common::{PrintToConsoleJob, set_up};

    #[tokio::test]
    async fn it_should_add_job_to_table() {
        set_up();

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
}
