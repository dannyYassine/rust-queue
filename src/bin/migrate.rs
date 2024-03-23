use dotenvy::dotenv;
use rust_queue::{
    models::{app_state::AppStateManager, data_connection::DatabaseConnection},
    repositories::job_repository::JobRepository,
};

#[tokio::main]
async fn main() {
    dotenv().ok();

    let connection = DatabaseConnection::create().await;
    AppStateManager::get_instance()
        .initialize()
        .set_connection(connection);

    println!("Running migrations");

    let job_repo = JobRepository::new().await;

    job_repo.create_table().await;
}
