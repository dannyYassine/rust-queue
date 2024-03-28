use std::time::Duration;

use axum::Json;
use rust_queue::models::{
    application::Application,
    router::{Controller, Route, Router},
};
use serde_json::{json, Value};

#[derive(Default)]
struct RootController;
impl Controller for RootController {
    fn execute(&self) -> Json<Value> {
        return Json(json!({"data": Duration::from_secs(1)}));
    }
}

struct ApiRouter;
impl Router for ApiRouter {
    fn register_routes() {
        Route::get::<RootController>("/");
        Route::get::<RootController>("/json");
        Route::post::<RootController>("/json");
        Route::put::<RootController>("/json");
        Route::delete::<RootController>("/json");
    }
}

#[tokio::main]
async fn main() {
    Application::shared()
        .register_routes::<ApiRouter>()
        .serve()
        .await;
}
