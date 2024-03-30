use sqlx::{Postgres, Transaction};

use crate::models::{
    app_state::AppStateManager,
    job::{Job, JobStatus},
};

#[derive(Default)]
pub struct JobRepository {}

#[allow(dead_code)]
impl JobRepository {
    pub fn new() -> Self {
        JobRepository {}
    }
    pub async fn create_table(&self) {
        let state = AppStateManager::get_instance().get_state();
        let connection = state.connection.as_ref().unwrap();

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
        .execute(connection)
        .await;
    }
    pub async fn add_job(&self, job: &Job) {
        let state = AppStateManager::get_instance().get_state();
        let connection = state.connection.as_ref().unwrap();

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
        .execute(connection)
        .await;
    }
    pub async fn get_first_pending_job(&self) -> Option<(Job, Transaction<'_, Postgres>)> {
        let state = AppStateManager::get_instance().get_state();
        let connection = state.connection.as_ref().unwrap();
        let mut tx = connection.begin().await.unwrap();

        let result: Result<Job, _> = sqlx::query_as::<_, Job>(
            format!(
                "SELECT id, payload, status, model_type, data FROM jobs where status = '{}'",
                JobStatus::Pending.to_string()
            )
            .as_str(),
        )
        .fetch_one(&mut *tx)
        .await;

        if result.is_err() {
            return None;
        }

        return Some((result.unwrap(), tx));
    }
    pub async fn get_all_jobs(&self, job_status: Option<JobStatus>) -> Option<Vec<Job>> {
        // let state = AppStateManager::get_instance().state();
        // let app_state = state.lock().unwrap();
        // let connection = app_state.connection.as_ref().unwrap();
        // let mut tx = connection.begin().await.unwrap();

        let connection = {
            let state = AppStateManager::get_instance().state();
            let app_state = state.lock().unwrap(); // Lock the mutex

            // Clone the connection to ensure it can be moved to the async block
            let connection = app_state.connection.as_ref().unwrap().clone();

            connection
        };
        let mut tx = connection.begin().await.unwrap();
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
    pub async fn update_job(&self, job: &Job, job_status: JobStatus) {
        let state = AppStateManager::get_instance().get_state();
        let connection = state.connection.as_ref().unwrap();

        let _ = sqlx::query(
            format!(
                "UPDATE jobs set status='{}' WHERE id = '{}'",
                job_status.to_string(),
                job.id
            )
            .as_str(),
        )
        .execute(connection)
        .await;
    }
    pub async fn delete_all_jobs(&self) {
        let state = AppStateManager::get_instance().get_state();
        let connection = state.connection.as_ref().unwrap();

        let _ = sqlx::query("DELETE from jobs;").execute(connection).await;
    }
}
