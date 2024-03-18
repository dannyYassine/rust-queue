use rust_queue::models::queue::Queue;
use dotenvy::dotenv;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let mut queue: Queue = Queue::new();
    queue.listen().await;
}
