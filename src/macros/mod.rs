#[macro_export]
macro_rules! dispatch {
    ($job:expr) => {
        use dotenvy::dotenv;
        use rust_queue::models::job::Job;
        use rust_queue::repositories::job_repository::JobRepository;
        use std::any::type_name_of_val;
        use std::env::{self, VarError};

        dotenv().ok();

        let job_repository = JobRepository::new();

        let s = type_name_of_val(&$job).to_string();
        let word = s.split("::").last().unwrap_or_default();

        let job: Job = Job::new(
            "".to_string(),
            "pending".to_string(),
            word.to_string(),
            serde_json::to_string(&$job).unwrap(),
        );

        job_repository.add_job(&job).await;
    };
}
