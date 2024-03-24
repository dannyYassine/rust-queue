use super::{app_state::AppStateManager, data_connection::DatabaseConnection};

pub struct Application {}

impl Application {
    pub async fn bootstrap() {
        Self::set_up_database_connection().await;
    }

    async fn set_up_database_connection() {
        let connection = DatabaseConnection::create();
        AppStateManager::get_instance().set_connection(connection);
    }
}
