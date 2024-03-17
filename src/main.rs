mod models;
use models::job::Job;
use models::queue::Queue;

use dotenv::dotenv;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let job: Job = Job {
        id: 1,
        payload: String::from(""),
    };

    let mut queue: Queue = Queue::new();
    queue.bootstrap().listen().await;

    println!("Hello, world!");
    println!("Job: {:?}", job);
}
