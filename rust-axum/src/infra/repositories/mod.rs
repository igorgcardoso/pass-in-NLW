mod attendee;
mod event;

pub trait Repository {
    type Model;
    type Id;
    type Error;

    async fn create(&self, model: &Self::Model) -> Result<Self::Model, Self::Error>;
    async fn get(&self, id: &Self::Id) -> Result<Self::Model, Self::Error>;
}

pub use self::{attendee::AttendeeRepository, event::EventRepository};
