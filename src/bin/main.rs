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

#[derive(Debug, Serialize, Deserialize)]
struct MyEvent {
    data: i32,
}

#[derive(Debug, Serialize, Deserialize)]
struct CustomJob(MyEvent);
impl JobHandle for CustomJob {
    fn handle(&self) {
        println!("Executing custom job: {:?}", self.0.data);
    }
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    Application::bootstrap().await;

    let mut queue: Queue = Queue::new()
        .register::<PrintToConsoleJob>()
        .register::<MultipleValueJob>()
        .register::<CustomJob>();
    queue.listen().await;
}
