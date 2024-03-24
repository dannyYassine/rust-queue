use dotenvy::dotenv;
use rust_queue::models::app_state::AppStateManager;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let app_state_manager = AppStateManager::get_instance();

    let state = app_state_manager.get_state();

    println!("AppState: {:?}", state.connection);
}
