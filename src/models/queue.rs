use super::job::{Job, JobStatus};
use sqlx::{PgPool, Postgres, Transaction};
use std::{
    env::{self, VarError},
    io::Error,
};
use tokio::time::{sleep, Duration};

pub struct Queue {
    connection: Option<PgPool>,
}

impl Queue {
    pub fn new() -> Self {
        return Queue { connection: None };
    }
    pub async fn bootstrap(&mut self) -> &Self {
        let result: Result<String, VarError> = env::var("DATABASE_URL");

        if result.is_err() {
            panic!("Missing DATABASE_URL");
        }

        let pool = sqlx::PgPool::connect(&result.unwrap()).await.unwrap();
        self.connection = Some(pool);

        return self;
    }
    pub async fn listen(&self) {
        println!("Processing jobs from the [default] queue.");

        loop {
            let tx = self.connection.unwrap().begin().await.unwrap();

            let mut job: Job = self.fetch_candidate_job(&tx).await;

            job.set_status_as_running();
            self.mark_job_as_running(&job);

            tx.commit().await.unwrap();

            let result: Result<_, _> = self.execute_job(&job).await;

            if result.is_err() {
                job.set_status_as_pending();
                self.mark_job_as_pending(&job);
                continue;
            }

            job.set_status_as_completed();
            self.mark_job_as_completed(&job);
        }
    }
    async fn fetch_candidate_job(&self, tx: &Transaction<'_, Postgres>) -> Job {
        let job: Job = sqlx::query!("SELECT id, payload FROM jobs WHERE status = 'pending'")
            .fetch_one(&tx)
            .await
            .unwrap();

        return job;
    }
    async fn mark_job_as_pending(&self, job: &Job) {
        self.mark_job_as_status(job, JobStatus::Pending);
    }
    async fn mark_job_as_running(&self, job: &Job) {
        self.mark_job_as_status(job, JobStatus::Running);
    }
    async fn mark_job_as_completed(&self, job: &Job) {
        self.mark_job_as_status(job, JobStatus::Completed);
    }
    async fn mark_job_as_status(&self, job: &Job, job_status: JobStatus) {
        sqlx::query!(
            "UPDATE jobs set status='$1' WHERE id = $2",
            job_status,
            job.id
        )
        .execute(&self.connection.unwrap())
        .await
        .unwrap();
    }
    async fn execute_job(&self, job: &Job) -> Result<bool, Error> {
        println!("Processing job {} started", job.id);

        // execute job
        let connection = self.connection.unwrap().clone();
        let handle = tokio::spawn(||self.process_job(&connection, job));
        handle.await?;

        let failed = false;
        if failed {
            println!("Processing job {} failed", job.id);
            return Err(Error::other("Job failed"));
        }

        println!("Processing job {} completed", job.id);
        return Ok(true);
    }
    async fn process_job(&self, connection: &PgPool, job: &Job) {
        sleep(Duration::from_secs(2)).await;
    }
}
