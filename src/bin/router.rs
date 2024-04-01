use async_trait::async_trait;
use axum::{response::Html, Json};
use serde::Deserialize;

use rust_queue::{
    json,
    models::{
        app_state::AppStateManager,
        application::Application,
        job::{Job, JobStatus},
        request::Request,
        resource::{JsonResource, Resource, ResourceArray},
        router::{Controller, Route, Router},
        template::{render_view, Template},
    },
    repositories::job_repository::JobRepository,
    view,
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
    type ReturnType = Json<Vec<User>>;

    async fn execute(&self, mut request: Request) -> Self::ReturnType {
        let param = request.payload::<UserParams>().await;
        println!("{:?}", param);

        let query_params = request.get_query_params();
        println!("{:?}", query_params.get::<String>("name"));
        println!("{:?}", request.get_query_params());

        let params: User = request.parse_into::<User>();

        return Json(vec![params, self.get_user().await]);
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

#[derive(Default, Debug)]
struct UserResource;

impl Resource<User> for UserResource {
    fn to_array(&self, data: User) -> ResourceArray {
        json! {
            "name" => data.name,
            "count" => 1
        }
    }
}

#[derive(Default)]
struct AdminRootController;
#[async_trait]
impl Controller for AdminRootController {
    type ReturnType = JsonResource;

    async fn execute(&self, _: Request) -> Self::ReturnType {
        let user = User {
            name: "".to_owned(),
            email: "a".to_owned(),
        };

        return UserResource::make(user);
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
    type ReturnType = Json<Vec<Job>>;

    async fn execute(&self, _: Request) -> Self::ReturnType {
        let results = self
            .job_repository
            .get_all_jobs(Some(JobStatus::Pending))
            .await;

        match results {
            Some(jobs) => Json(jobs),
            None => Json(vec![]),
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
impl Controller for RenderHtmlController {
    type ReturnType = Html<String>;

    async fn execute(&self, _: Request) -> Self::ReturnType {
        {
            let mut state = AppStateManager::shared().get_state();
            state.counter += 1;
        }

        return view!(
            "index.html",
            RenderHtmlData {
                count: AppStateManager::shared().get_state().counter,
            }
        );

        // return Html(Template::render::<RenderHtmlData>(
        //     "index.html",
        //     RenderHtmlData {
        //         count: AppStateManager::shared().get_state().counter,
        //     },
        // ));
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
        Route::get::<RenderHtmlController>("/html");
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
