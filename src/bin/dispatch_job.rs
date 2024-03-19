use dotenvy::dotenv;
use rust_queue::{dispatch, models::job::Job};

#[tokio::main]
async fn main() {
    dotenv().ok();

    for _ in 0..5 {
        let job: Job = Job {
            id: 1,
            payload: "".to_string(),
            status: "pending".to_string(),
            model_type: "rust_queue::models::job::Job".to_string(),
        };

        println!("Dispatch job");
        dispatch!(job);
    }
}
