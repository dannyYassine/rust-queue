#![allow(unsafe_code)]

use crate::models::job::{Job, JobStatus};
use crate::repositories::job_repository::JobRepository;
use serde::{Deserialize, Serialize};
use sqlx::Transaction;
use std::any::type_name;
use std::collections::HashMap;
use std::io::Error;
use std::sync::mpsc::{self, Sender};
use tokio::time::{sleep, Duration};

use super::job::JobHandle;

// Type alias for the closure used in the JobMap
type JobClosure = Box<dyn Fn(String) -> Box<dyn JobHandle>>;

// HashMap for the JobMap
type JobMap = HashMap<String, JobClosure>;

pub struct Queue {
    job_limit: Option<i32>,
    map: JobMap,
    job_repository: JobRepository,
}

impl Queue {
    pub fn new() -> Self {
        return Queue {
            job_limit: None,
            map: HashMap::new(),
            job_repository: JobRepository::new(),
        };
    }
    pub fn register<J>(mut self) -> Self
    where
        J: JobHandle + Serialize + for<'de> Deserialize<'de>,
    {
        let s = type_name::<J>().to_owned();
        let job_key = s.split("::").last().unwrap_or_default();

        self.map.insert(
            job_key.to_owned(),
            Box::new(move |json_value: String| {
                Box::new(serde_json::from_str::<J>(json_value.as_str()).unwrap())
                    as Box<dyn JobHandle>
            }),
        );

        return self;
    }
    pub fn new_with_job_limit(job_limit: i32) -> Self {
        return Queue {
            job_limit: Some(job_limit),
            map: HashMap::new(),
            job_repository: JobRepository::new(),
        };
    }
    pub async fn listen(&mut self) {
        let mut count: i32 = 0;

        println!("Processing jobs from the [default] queue.");

        loop {
            let result: Option<(Job, Transaction<'_, _>)> =
                self.job_repository.get_first_pending_job().await;

            if result.is_none() {
                println!("No jobs found, retrying in 1 second");
                sleep(Duration::from_secs(1)).await;
                continue;
            }
            let (mut job, tx) = result.unwrap();

            if let Some(job_handle) = self.map.get(job.model_type.as_str()) {
                job_handle(job.data.to_owned()).handle();
            }
            job.set_status_as_running();
            self.mark_job_as_running(&job).await;
            tx.commit().await.unwrap();

            let result: Result<_, _> = self.execute_job(&job).await;

            if result.is_err() {
                job.set_status_as_pending();
                self.mark_job_as_pending(&job).await;
                continue;
            }

            count += 1;
            job.set_status_as_completed();
            self.mark_job_as_completed(&job).await;

            match self.job_limit {
                Some(val) => {
                    if count >= val {
                        break;
                    }
                }
                None => continue,
            }
        }
    }
    async fn mark_job_as_pending(&self, job: &Job) {
        self.mark_job_as_status(job, JobStatus::Pending).await;
    }
    async fn mark_job_as_running(&self, job: &Job) {
        self.mark_job_as_status(job, JobStatus::Running).await;
    }
    async fn mark_job_as_completed(&self, job: &Job) {
        self.mark_job_as_status(job, JobStatus::Completed).await;
    }
    async fn mark_job_as_status(&self, job: &Job, job_status: JobStatus) {
        self.job_repository.update_job(job, job_status).await;
    }
    async fn execute_job(&self, job: &Job) -> Result<bool, Error> {
        println!("Processing job {} started", job.id);

        let (sender, receiver) = mpsc::channel::<bool>();
        let job_to_be_processed: Job = job.clone();
        let handle = tokio::spawn(process_job(sender, job_to_be_processed));
        handle.await?;

        let result: Result<bool, _> = receiver.recv();

        if result.is_err() {
            println!("Processing job {} failed", job.id);
            if let Err(error) = result {
                println!("with error:: {}", error);
            }
            return Err(Error::other("Job failed"));
        }

        let failed: bool = result.unwrap();
        if !failed {
            println!("Processing job {} failed", job.id);
            return Err(Error::other("Job failed"));
        }
        println!("Processing job {} completed", job.id);

        return Ok(true);
    }
}

async fn process_job(sender: Sender<bool>, job: Job) {
    let handle_job = || -> Result<(), Error> {
        job.handle();
        return Ok(());
    };

    let result: Result<(), mpsc::SendError<bool>>;
    if let Err(_) = handle_job() {
        result = sender.send(false);
    } else {
        result = sender.send(true);
    }

    if result.is_err() {
        return println!("{:?}", result);
    }

    result.unwrap();
}
