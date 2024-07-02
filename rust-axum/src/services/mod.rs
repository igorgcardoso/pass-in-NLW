pub trait ServiceTrait {
    type Model;
    type Error;
    type Id;

    // async fn list(&self) -> Result<Vec<Self::Model>, Self::Error>;
    async fn get(&self, id: &Self::Id) -> Result<Self::Model, Self::Error>;
    async fn create(&self, model: &Self::Model) -> Result<Self::Model, Self::Error>;
}

mod attendee;
mod event;

use deadpool_diesel::postgres::Pool;

#[derive(Clone)]
pub struct Services {
    pub event: event::EventService,
    pub attendee: attendee::AttendeeService,
}

impl Services {
    pub fn new(pool: Box<Pool>) -> Self {
        Self {
            event: event::EventService::new(pool.clone()),
            attendee: attendee::AttendeeService::new(pool),
        }
    }
}
