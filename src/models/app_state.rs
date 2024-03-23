use std::sync::{Mutex, MutexGuard};

use sqlx::PgPool;

#[derive(Debug)]
pub struct AppState {
    pub connection: Option<PgPool>,
}

impl AppState {
    pub fn new() -> Self {
        AppState { connection: None }
    }
}

pub struct AppStateManager {
    state: Mutex<Option<AppState>>,
}

impl AppStateManager {
    pub fn get_instance() -> &'static Self {
        static INSTANCE: AppStateManager = AppStateManager {
            state: Mutex::new(None),
        };

        return &INSTANCE;
    }

    pub fn initialize(&self) -> &Self {
        let mut guard = self.state.lock().unwrap();
        *guard = Some(AppState::new());

        return self;
    }

    pub fn set_connection(&self, connection: PgPool) {
        let mut guard = self.state.lock().unwrap();
        if let Some(ref mut state) = *guard {
            state.connection = Some(connection);
        }
    }

    pub fn get_state(&self) -> MutexGuard<'_, Option<AppState>> {
        return self.state.lock().unwrap();
    }
}
