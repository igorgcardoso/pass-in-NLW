use validator::Validate;

use super::Event;

#[derive(Debug, Clone, Validate)]
pub struct Attendee {
    id: i32,
    #[validate(length(min = 4))]
    name: String,
    #[validate(email)]
    email: String,
    event: Event,
    created_at: chrono::NaiveDateTime,
    checked_in_at: Option<chrono::NaiveDateTime>,
}

impl Attendee {
    pub fn new(
        id: Option<i32>,
        name: String,
        email: String,
        event: Event,
        created_at: Option<chrono::NaiveDateTime>,
        checked_in_at: Option<chrono::NaiveDateTime>,
    ) -> Self {
        let id = id.unwrap_or(0);
        Attendee {
            id,
            name,
            email,
            event,
            created_at: created_at.unwrap_or_else(|| chrono::Local::now().naive_local()),
            checked_in_at,
        }
    }

    pub fn set_id(&mut self, id: i32) {
        self.id = id;
    }

    pub fn id(&self) -> i32 {
        self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn email(&self) -> &str {
        &self.email
    }

    pub fn event(&self) -> Event {
        self.event.clone()
    }

    pub fn created_at(&self) -> chrono::NaiveDateTime {
        self.created_at
    }

    pub fn checked_in_at(&self) -> Option<chrono::NaiveDateTime> {
        self.checked_in_at
    }
}
