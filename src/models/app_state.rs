use std::sync::{Arc, Mutex, MutexGuard};

use lazy_static::lazy_static;
use sqlx::PgPool;

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
        AppStateManager {
            state: Arc::new(Mutex::new(AppState { connection: None })),
        }
    };
}

impl AppStateManager {
    pub fn get_instance() -> &'static Self {
        &APP_STATE_MANAGER
    }

    pub fn state(&self) -> Arc<Mutex<AppState>> {
        return Arc::clone(&self.state);
    }

    pub fn get_state(&self) -> MutexGuard<'_, AppState> {
        return self.state.lock().unwrap();
    }

    pub fn set_connection(&self, connection: PgPool) {
        let mut guard = self.state.lock().unwrap();
        guard.connection = Some(connection);
    }
}
