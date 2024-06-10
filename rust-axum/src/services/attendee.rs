use std::collections::HashSet;

use deadpool_diesel::sqlite::Pool;
use uuid::Uuid;
use validator::Validate;

use crate::{
    domain::Attendee,
    error::AppError,
    infra::{AttendeeRepository, Repository},
};

use super::ServiceTrait;

#[derive(Clone)]
pub struct AttendeeService {
    repository: AttendeeRepository,
}

impl ServiceTrait for AttendeeService {
    type Id = i32;
    type Model = Attendee;
    type Error = AppError;

    async fn get(&self, id: &Self::Id) -> Result<Self::Model, Self::Error> {
        self.repository.get(id).await
    }

    async fn create(&self, model: &Self::Model) -> Result<Self::Model, Self::Error> {
        model.validate()?;

        let event = model.event();

        if let Some(maximum_attendees) = event.maximum_attendees() {
            if event.attendees_amount() >= maximum_attendees {
                return Err(AppError::BadRequest("Event is full.".to_string()));
            }
        }

        let mut has_next = true;
        let mut page_index = 0;
        let page_size = 4096;

        while has_next {
            let (attendees, total) = self
                .repository
                .get_from_event(event.id(), None, page_index, page_size)
                .await?;

            has_next = total > (page_index + 1) * page_size;
            page_index += 1;

            if attendees
                .iter()
                .map(|attendee| attendee.email())
                .collect::<HashSet<_>>()
                .contains(model.email())
            {
                return Err(AppError::BadRequest(
                    "Attendee already registered.".to_string(),
                ));
            }
        }

        self.repository.create(model).await
    }
}

impl AttendeeService {
    pub fn new(pool: Box<Pool>) -> Self {
        let repository = AttendeeRepository::new(pool.clone());
        Self { repository }
    }

    pub async fn get_from_event(
        &self,
        event_id: Uuid,
        filter: Option<String>,
        page_index: i64,
        page_size: i64,
    ) -> Result<(Vec<Attendee>, i64), AppError> {
        self.repository
            .get_from_event(event_id, filter, page_index, page_size)
            .await
    }

    pub async fn check_in(&self, attendee_id: i32) -> Result<(), AppError> {
        let attendee = self.get(&attendee_id).await?;

        if attendee.checked_in_at().is_some() {
            return Err(AppError::BadRequest(
                "Attendee already checked in.".to_string(),
            ));
        }

        self.repository.check_in(&attendee).await
    }
}
