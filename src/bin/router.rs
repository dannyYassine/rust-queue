use async_trait::async_trait;
use rust_queue::{
    models::{
        application::Application,
        job::Job,
        router::{Controller, Route, Router},
    },
    repositories::job_repository::JobRepository,
};
use serde::Serialize;

#[derive(Serialize)]
struct Data(&'static str);

#[derive(Default)]
struct RootController;

#[async_trait]
impl Controller for RootController {
    type ReturnType = Data;

    async fn execute(&self) -> Self::ReturnType {
        return Data("hello");
    }
}
#[derive(Default)]
struct AdminRootController;
#[async_trait]
impl Controller for AdminRootController {
    type ReturnType = Data;

    async fn execute(&self) -> Self::ReturnType {
        return Data("admin");
    }
}

// #[derive(Default)]
// struct GetJobsController;
// impl Controller for GetJobsController {
//     type ReturnType = Vec<Job>;

//     fn execute(&self) -> Self::ReturnType {
//         let jobs = JobRepository::new().get_all_jobs(None).await.unwrap();

//         return jobs;
//     }
// }

struct ApiRouter;
impl Router for ApiRouter {
    fn register_routes() {
        Route::get::<RootController>("/");
        Route::get::<RootController>("/jobs");
        Route::post::<RootController>("/json");
        Route::put::<RootController>("/json");
        Route::delete::<RootController>("/json");

        Route::group("/admin", || {
            Route::get::<AdminRootController>("/");
        });

        Route::get::<RootController>("/data");
    }
}

#[tokio::main]
async fn main() {
    Application::shared()
        .register_routes::<ApiRouter>()
        .serve()
        .await;
}
