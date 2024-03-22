use std::env::{self, VarError};

use sqlx::{PgPool, Postgres, Transaction};

use crate::models::job::{Job, JobStatus};

pub struct JobRepository {
    connection: PgPool,
}

#[allow(dead_code)]
impl JobRepository {
    pub async fn new() -> Self {
        JobRepository {
            connection: Self::bootstrap().await,
        }
    }
    pub async fn create_table(&self) {
        let _ = sqlx::query(
            format!(
                "CREATE TABLE jobs (
                    id SERIAL PRIMARY KEY,
                    payload TEXT NOT NULL,
                    status VARCHAR(20) NOT NULL DEFAULT 'pending',
                    model_type TEXT NOT NULL,
                    data TEXT NOT NULL
                );"
            )
            .as_str(),
        )
        .execute(&self.connection)
        .await;
    }
    pub async fn add_job(&self, job: &Job) {
        let _ = sqlx::query(
            format!(
                "INSERT INTO jobs (payload, status, model_type, data) VALUES ('{}', '{}', '{}', '{}');",
                job.payload,
                job.status,
                job.model_type,
                job.data
            )
            .as_str(),
        )
        .execute(&self.connection)
        .await;
    }
    pub async fn get_first_pending_job(&self) -> Option<(Job, Transaction<'_, Postgres>)> {
        let mut tx = self.connection.begin().await.unwrap();

        let result: Result<Job, _> = sqlx::query_as::<_, Job>(
            "SELECT id, payload, status, model_type, data FROM jobs where status = {}",
        )
        .bind(JobStatus::Pending.to_string())
        .fetch_one(&mut *tx)
        .await;

        if result.is_err() {
            return None;
        }

        return Some((result.unwrap(), tx));
    }
    pub async fn get_all_jobs(&self, job_status: Option<JobStatus>) -> Option<Vec<Job>> {
        let mut tx = self.connection.begin().await.unwrap();

        let status = match job_status {
            Some(status) => status,
            _ => JobStatus::Pending,
        };

        let results: Result<Vec<Job>, _> = sqlx::query_as::<_, Job>(
            format!(
                "SELECT id, payload, status, model_type, data FROM jobs where status = '{}'",
                status.to_string()
            )
            .as_str(),
        )
        .fetch_all(&mut *tx)
        .await;

        if results.is_err() {
            return None;
        }

        return Some(results.unwrap());
    }
    pub async fn delete_all_jobs(&self) {
        let _ = sqlx::query("DELETE from jobs;")
            .execute(&self.connection)
            .await;
    }
    pub async fn bootstrap() -> sqlx::Pool<Postgres> {
        let result: Result<String, VarError> = env::var("DATABASE_URL");

        if result.is_err() {
            panic!("Missing DATABASE_URL");
        }

        return sqlx::PgPool::connect(&result.unwrap()).await.unwrap();
    }
}
