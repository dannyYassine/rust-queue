use dotenvy::dotenv;
use rust_queue::models::{application::Application, job::JobHandle, queue::Queue};
use serde::{Deserialize, Serialize};

impl JobHandle for PrintToConsoleJob {
    fn handle(&self) {
        println!("running PrintToConsoleJob");
    }
}

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

    Application::bootstrap().await;

    let mut queue: Queue = Queue::new()
        .register::<PrintToConsoleJob>()
        .register::<MultipleValueJob>();
    queue.listen().await;
}
