use std::default;

use async_trait::async_trait;
use axum::{
    body::Body,
    extract::{Query, Request},
    http::request,
};
use rust_queue::{
    models::{
        app_state::AppStateManager,
        application::Application,
        job::{Job, JobStatus},
        router::{Controller, Route, Router},
    },
    repositories::job_repository::JobRepository,
};
use serde::Serialize;

#[derive(Serialize)]
struct Data(&'static str);

#[derive(Serialize)]
struct User {
    name: String,
    email: String,
}

#[derive(Default)]
struct RootController;
#[async_trait]
impl Controller for RootController {
    type ReturnType = User;

    async fn execute(&self, request: Request<Body>) -> Self::ReturnType {
        return self.get_user().await;
    }
}

impl RootController {
    async fn get_user(&self) -> User {
        return User {
            name: String::from("Danny"),
            email: String::from("yo@gmail.com"),
        };
    }
}
#[derive(Default)]
struct AdminRootController;
#[async_trait]
impl Controller for AdminRootController {
    type ReturnType = Data;

    async fn execute(&self, _: Request<Body>) -> Self::ReturnType {
        return Data("admin");
    }
}

#[derive(Default)]
struct GetJobsController {
    job_repository: JobRepository,
}
impl GetJobsController {
    pub fn default() -> Self {
        GetJobsController {
            job_repository: JobRepository::default(),
        }
    }
}
#[async_trait]
impl Controller for GetJobsController {
    type ReturnType = Vec<Job>;

    async fn execute(&self, _: Request<Body>) -> Self::ReturnType {
        let results = self
            .job_repository
            .get_all_jobs(Some(JobStatus::Pending))
            .await;

        match results {
            Some(jobs) => jobs,
            None => vec![],
        }
    }
}

#[derive(Default)]
struct GetHealthController;
#[async_trait]
impl Controller for GetHealthController {
    type ReturnType = String;

    async fn execute(&self, _: Request<Body>) -> Self::ReturnType {
        {
            let mut state = AppStateManager::shared().get_state();
            state.counter += 1;
        }

        return format!(
            "{}, and count is {}",
            "Alive".to_owned(),
            AppStateManager::shared().get_state().counter
        );
    }
}

struct ApiRouter;
impl Router for ApiRouter {
    fn register_routes() {
        Route::get::<RootController>("/");
        Route::get::<GetJobsController>("/jobs");

        Route::group("/admin", || {
            Route::get::<AdminRootController>("/");
        });

        Route::get::<GetHealthController>("/health");
    }
}

#[tokio::main]
async fn main() {
    Application::shared()
        .initialize()
        .await
        .register_routes::<ApiRouter>()
        .await
        .serve()
        .await;
}
