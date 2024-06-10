use deadpool_diesel::sqlite::Pool;

use crate::services::Services;

#[derive(Clone)]
pub struct AppState {
    pub services: Services,
}

impl AppState {
    pub fn new(pool: Box<Pool>) -> Self {
        let services = Services::new(pool);
        Self { services }
    }
}
