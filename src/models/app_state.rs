use std::sync::{Arc, Mutex, MutexGuard};

use lazy_static::lazy_static;
use sqlx::PgPool;

use super::data_connection::DatabaseConnection;

#[derive(Debug)]
pub struct AppState {
    pub connection: Option<PgPool>,
}

impl AppState {
    pub fn new(connection: PgPool) -> Self {
        AppState {
            connection: Some(connection),
        }
    }
}

pub struct AppStateManager {
    state: Arc<Mutex<AppState>>,
}

lazy_static! {
    static ref APP_STATE_MANAGER: AppStateManager = {
        let connection = DatabaseConnection::create();
        AppStateManager {
            state: Arc::new(Mutex::new(AppState::new(connection))),
        }
    };
}

impl AppStateManager {
    pub fn get_instance() -> &'static Self {
        &APP_STATE_MANAGER
    }

    pub fn get_state(&self) -> MutexGuard<'_, AppState> {
        return self.state.lock().unwrap();
    }
}
