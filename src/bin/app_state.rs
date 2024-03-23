use std::env::{self, VarError};

use dotenvy::dotenv;
use rust_queue::models::app_state::AppStateManager;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let result: Result<String, VarError> = env::var("DATABASE_URL");

    let connection = sqlx::PgPool::connect(&result.unwrap()).await.unwrap();

    let app_state_manager = AppStateManager::get_instance();
    app_state_manager.initialize();
    app_state_manager.set_connection(connection);

    let state = app_state_manager.get_state();

    let app_state = state.as_ref().unwrap();
    println!("AppState: {:?}", app_state.connection);
}
