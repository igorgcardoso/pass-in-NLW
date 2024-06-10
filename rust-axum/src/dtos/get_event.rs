use serde::Serialize;
use utoipa::ToSchema;

#[derive(Serialize, ToSchema)]
pub struct GetEventDTO {
    pub event: EventDTO,
}

#[derive(Serialize, ToSchema)]
pub struct EventDTO {
    pub id: uuid::Uuid,
    pub title: String,
    pub details: Option<String>,
    pub slug: String,
    pub maximum_attendees: Option<i32>,
    pub attendees_amount: i32,
}
