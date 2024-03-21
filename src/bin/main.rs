use dotenvy::dotenv;
use rust_queue::models::{custom_job::PrintToConsoleJob, queue::Queue};

#[tokio::main]
async fn main() {
    dotenv().ok();

    let mut queue: Queue = Queue::new().register::<PrintToConsoleJob>();
    queue.listen().await;
}
