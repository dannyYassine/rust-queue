use dotenvy::dotenv;
use rust_queue::{models::application::Application, repositories::job_repository::JobRepository};

#[tokio::main]
async fn main() {
    dotenv().ok();

    Application::bootstrap().await;

    println!("Running migrations");

    let job_repo = JobRepository::new();

    job_repo.create_table().await;
}
