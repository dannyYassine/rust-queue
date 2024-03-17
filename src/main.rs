mod models;
use dotenvy::dotenv;
use models::queue::Queue;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let mut queue: Queue = Queue::new();
    queue.bootstrap().await.listen().await;
}
