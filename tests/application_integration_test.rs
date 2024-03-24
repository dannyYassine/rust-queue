use rust_queue::models::{app_state::AppStateManager, application::Application};

mod common;
use common::set_up;

#[tokio::test]
async fn it_should_create_connection_into_app_state_manager() {
    set_up();

    Application::bootstrap().await;

    assert!(AppStateManager::get_instance()
        .get_state()
        .connection
        .is_some());
}
