use dotenvy::dotenv;
use rust_queue::models::app_state::AppStateManager;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let app_state_manager = AppStateManager::get_instance();

    let state = app_state_manager.get_state();

    let app_state = state.as_ref();
    println!("AppState: {:?}", app_state.connection);
}
