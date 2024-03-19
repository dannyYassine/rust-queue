use dotenvy::dotenv;
use rust_queue::models::queue::Queue;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let mut queue: Queue = Queue::new();
    queue.listen().await;
}
