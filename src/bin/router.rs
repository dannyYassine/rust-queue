use async_trait::async_trait;
use axum::{response::Html, Json};
use serde::Deserialize;

use rust_queue::{
    models::{
        app_state::{AppState, AppStateManager},
        application::Application,
        job::{Job, JobStatus},
        request::Request,
        router::{Controller, HtmlController, Route, Router},
        template::Template,
    },
    repositories::job_repository::JobRepository,
};
use serde::Serialize;

#[derive(Serialize)]
struct Data(&'static str);

#[derive(Debug, Serialize, Deserialize)]
struct User {
    name: String,
    email: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct UserParams {
    #[serde(default)]
    name: String,
    #[serde(default)]
    email: String,
}

#[derive(Default)]
struct RootController;
#[async_trait]
impl Controller for RootController {
    type ReturnType = Vec<User>;

    async fn execute(&self, mut request: Request) -> Self::ReturnType {
        let param = request.payload::<UserParams>().await;
        println!("{:?}", param);

        let query_params = request.get_query_params();
        println!("{:?}", query_params.get::<String>("name"));
        println!("{:?}", request.get_query_params());

        let params: User = request.parse_into::<User>();

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
    type ReturnType = Data;

    async fn execute(&self, _: Request) -> Self::ReturnType {
        return Data("admin");
    }
}

#[derive(Default)]
struct GetJobsController {
    job_repository: JobRepository,
}
impl GetJobsController {
    #[allow(dead_code)]
    pub fn default() -> Self {
        GetJobsController {
            job_repository: JobRepository::default(),
        }
    }
}
#[async_trait]
impl Controller for GetJobsController {
    type ReturnType = Vec<Job>;

    async fn execute(&self, _: Request) -> Self::ReturnType {
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

    async fn execute(&self, _: Request) -> Self::ReturnType {
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

#[derive(Serialize)]
struct RenderHtmlData {
    count: u32,
}

#[derive(Default)]
struct RenderHtmlController;
#[async_trait]
impl HtmlController for RenderHtmlController {
    async fn execute(&self, _: Request) -> String {
        {
            let mut state = AppStateManager::shared().get_state();
            state.counter += 1;
        }

        return Template::render::<RenderHtmlData>(
            "index.html",
            RenderHtmlData {
                count: AppStateManager::shared().get_state().counter,
            },
        );
    }
}

struct ApiRouter;
impl Router for ApiRouter {
    fn register_routes() {
        Route::post::<RootController>("/");
        Route::get::<GetJobsController>("/jobs");

        Route::group("/admin", || {
            Route::get::<AdminRootController>("/");
        });

        Route::get::<GetHealthController>("/health");
        Route::html::<RenderHtmlController>("/html");
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
