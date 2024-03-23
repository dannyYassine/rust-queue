use std::env::{self, VarError};

use dotenvy::dotenv;
use rust_queue::models::{
    app_state::AppStateManager,
    job::{JobHandle, JobName},
    queue::Queue,
};
use serde::{Deserialize, Serialize};

impl JobName for PrintToConsoleJob {}
impl JobHandle for PrintToConsoleJob {
    fn handle(&self) {
        println!("running PrintToConsoleJob");
    }
}

impl JobName for MultipleValueJob {}
impl JobHandle for MultipleValueJob {
    fn handle(&self) {
        println!("running MultipleValueJob: {}", self.value * self.value);
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct PrintToConsoleJob {
    name: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct MultipleValueJob {
    value: i32,
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    let result: Result<String, VarError> = env::var("DATABASE_URL");
    let connection = sqlx::PgPool::connect(&result.unwrap()).await.unwrap();

    AppStateManager::get_instance()
        .initialize()
        .set_connection(connection);

    let mut queue: Queue = Queue::new()
        .register::<PrintToConsoleJob>()
        .register::<MultipleValueJob>();
    queue.listen().await;
}
