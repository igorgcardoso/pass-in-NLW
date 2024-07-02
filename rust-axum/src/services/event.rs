use deadpool_diesel::postgres::Pool;
use uuid::Uuid;
use validator::Validate;

use crate::{
    domain::Event,
    error::AppError,
    infra::{EventRepository, Repository},
};

use super::ServiceTrait;

#[derive(Clone)]
pub struct EventService {
    repository: EventRepository,
}

impl EventService {
    pub fn new(pool: Box<Pool>) -> Self {
        let repository = EventRepository::new(pool);
        Self { repository }
    }
}

impl ServiceTrait for EventService {
    type Model = Event;
    type Error = AppError;
    type Id = Uuid;

    /*
    async fn list(&self) -> Result<Vec<Self::Model>, Self::Error> {
        self.repository.list().await
    }
    */

    async fn get(&self, id: &Self::Id) -> Result<Self::Model, Self::Error> {
        self.repository.get(id).await
    }

    async fn create(&self, model: &Self::Model) -> Result<Self::Model, Self::Error> {
        model.validate()?;
        self.repository.create(model).await
    }
}
