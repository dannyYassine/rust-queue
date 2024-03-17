mod models;
use dotenv::dotenv;
use models::queue::Queue;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let mut queue: Queue = Queue::new();
    queue.bootstrap().await.listen().await;
}
