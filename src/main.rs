use dotenvy::dotenv;
use rust_queue::models::{job::Job, queue::Queue};
mod macros;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let job: Job = Job {
        id: 1,
        payload: "".to_string(),
        status: "pending".to_string(),
        model_type: "rust_queue::models::job::Job".to_string(),
    };
    dispatch!(job);

    let mut queue: Queue = Queue::new();
    queue.listen().await;
}
