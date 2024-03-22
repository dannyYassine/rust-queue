use dotenvy::dotenv;
use rust_queue::repositories::job_repository::JobRepository;

#[tokio::main]
async fn main() {
    dotenv().ok();

    println!("Running migrations");

    let job_repo = JobRepository::new().await;

    job_repo.create_table().await;
}
