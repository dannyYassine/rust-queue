use dotenvy::dotenv;

use super::{app_state::AppStateManager, data_connection::DatabaseConnection};

pub struct Application {}

impl Application {
    pub async fn bootstrap() {
        Self::load_env_vars().await;
        Self::set_up_database_connection().await;
    }

    async fn load_env_vars() {
        dotenv().ok();
    }

    async fn set_up_database_connection() {
        let connection = DatabaseConnection::create();
        AppStateManager::get_instance().set_connection(connection);
    }
}
