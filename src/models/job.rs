use std::any::type_name_of_val;

use serde::Serialize;

use crate::repositories::job_repository::JobRepository;

pub trait Dispatchable: Serialize {
    #[allow(async_fn_in_trait)]
    async fn dispatch(&self) {
        let job_repository = JobRepository::new();

        let s = type_name_of_val(self).to_string();
        let word = s.split("::").last().unwrap_or_default();

        let job: Job = Job::new(
            "".to_string(),
            "pending".to_string(),
            word.to_string(),
            serde_json::to_string(self).unwrap(),
        );

        job_repository.add_job(&job).await;
    }
}
// Trait for the method `handle`
pub trait JobHandle: 'static {
    fn handle(&self);
}

#[derive(Debug, Clone, sqlx::FromRow, Serialize)]
pub struct Job {
    pub id: i32,
    pub payload: String,
    pub status: String,
    pub model_type: String,
    pub data: String,
}

impl Job {
    pub fn handle(&self) {
        //
    }
    pub fn new(payload: String, status: String, model_type: String, data: String) -> Self {
        Job {
            id: 0,
            payload,
            status,
            model_type,
            data,
        }
    }
    pub fn set_status_as_pending(&mut self) -> &Self {
        self.status = JobStatus::Pending.to_string();

        return self;
    }
    pub fn set_status_as_running(&mut self) -> &Self {
        self.status = JobStatus::Running.to_string();

        return self;
    }
    pub fn set_status_as_completed(&mut self) -> &Self {
        self.status = JobStatus::Completed.to_string();

        return self;
    }
}

#[derive(Debug)]
pub enum JobStatus {
    Pending,
    Running,
    Completed,
}

impl JobStatus {
    pub fn to_string(&self) -> String {
        return match self {
            JobStatus::Pending => String::from("pending"),
            JobStatus::Running => String::from("running"),
            JobStatus::Completed => String::from("completed"),
        };
    }
}
