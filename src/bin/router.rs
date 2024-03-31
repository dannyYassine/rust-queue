use std::default;

use async_trait::async_trait;
use axum::{
    body::Body,
    extract::{Query, Request},
    http::request,
    Json,
};
use serde::Deserialize;

use rust_queue::{
    models::{
        app_state::AppStateManager,
        application::Application,
        job::{Job, JobStatus},
        request::Request as NewRequest,
        router::{Controller, Route, Router},
    },
    repositories::job_repository::JobRepository,
};
use serde::Serialize;
use serde_json::from_str;

#[derive(Serialize)]
struct Data(&'static str);

#[derive(Debug, Serialize, Deserialize)]
struct User {
    name: String,
    email: String,
}

#[derive(Default)]
struct RootController;
#[async_trait]
impl Controller for RootController {
    type RequestType<T> = Json<User>;
    type ReturnType = Vec<User>;

    async fn execute(&self, request: Request<Body>) -> Self::ReturnType {
        let uri = request.uri().clone();

        let req = NewRequest(request);

        let query_params = req.get_query_params();
        println!("{:?}", query_params.get::<String>("name"));
        println!("{:?}", req.get_query_params());

        // Parse the query parameters from the URI
        let query_params = uri.query().unwrap_or("");
        let query_params = query_params.split("::").last().unwrap();

        let params: User = serde_qs::from_str(query_params).unwrap();

        return vec![params, self.get_user().await];
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
    type RequestType<T> = Json<String>;
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
    type RequestType<T> = Json<String>;
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
    type RequestType<T> = Json<String>;
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
