use deadpool_diesel::postgres::Pool;

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
