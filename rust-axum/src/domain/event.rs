use uuid::Uuid;
use validator::Validate;

use crate::{dtos::CreateEventRequestDTO, utils::generate_slug};

#[derive(Debug, Clone, Validate)]
pub struct Event {
    id: Uuid,
    #[validate(length(min = 4))]
    title: String,
    details: Option<String>,
    slug: String,
    #[validate(range(min = 1))]
    maximum_attendees: Option<i32>,
    attendees_amount: i32,
}

impl Event {
    pub fn new(
        id: Option<Uuid>,
        title: String,
        details: Option<String>,
        slug: Option<String>,
        maximum_attendees: Option<i32>,
        attendees_amount: i32,
    ) -> Self {
        let id = id.unwrap_or_else(Uuid::new_v4);
        let slug = slug.unwrap_or_else(|| generate_slug(&title));
        Event {
            id,
            title,
            details,
            slug,
            maximum_attendees,
            attendees_amount,
        }
    }

    pub fn id(&self) -> Uuid {
        self.id
    }

    pub fn title(&self) -> &str {
        &self.title
    }

    pub fn details(&self) -> Option<&str> {
        self.details.as_deref()
    }

    pub fn slug(&self) -> &str {
        &self.slug
    }

    pub fn maximum_attendees(&self) -> Option<i32> {
        self.maximum_attendees
    }

    pub fn attendees_amount(&self) -> i32 {
        self.attendees_amount
    }
}

impl From<CreateEventRequestDTO> for Event {
    fn from(dto: CreateEventRequestDTO) -> Self {
        let title = dto.title.clone();
        Event {
            id: Uuid::new_v4(),
            title: dto.title,
            details: dto.details,
            slug: generate_slug(&title),
            maximum_attendees: dto.maximum_attendees,
            attendees_amount: 0,
        }
    }
}
