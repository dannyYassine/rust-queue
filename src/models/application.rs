use std::sync::{Arc, Mutex};

use dotenvy::dotenv;

use super::{
    app_state::AppStateManager, data_connection::DatabaseConnection,
    service_providers::ServiceProvider,
};

#[derive(Clone)]
pub struct Application {
    service_providers: Arc<Mutex<Vec<Box<dyn ServiceProvider>>>>,
}

impl Application {
    pub async fn init() -> Self {
        Application {
            service_providers: Arc::new(Mutex::new(vec![])),
        }
    }
    pub async fn register_root_service_provider<S>(&self) -> &Self
    where
        S: ServiceProvider + Default,
    {
        let root = S::default();

        {
            root.register(self);
            let providers = self.service_providers.lock().unwrap();
            for provider in providers.iter() {
                provider.register(self);
            }
        }
        {
            root.boot(self);
            let providers = self.service_providers.lock().unwrap();
            for provider in providers.iter() {
                provider.boot(self);
            }
        }

        return self;
    }
    pub fn register_service_provider<S>(&self)
    where
        S: ServiceProvider + Default + 'static,
    {
        let mut providers = self.service_providers.lock().unwrap();
        providers.push(Box::new(S::default()));
    }
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
