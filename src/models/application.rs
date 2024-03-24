use super::app_state::AppStateManager;

pub struct Application {}

impl Application {
    pub async fn bootstrap() {
        Self::set_up_database_connection().await;
    }

    async fn set_up_database_connection() {
        let _ = AppStateManager::get_instance();
    }
}
