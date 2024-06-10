use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct CreateEventRequestDTO {
    pub title: String,
    pub details: Option<String>,
    pub maximum_attendees: Option<i32>,
}

#[derive(Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct CreateEventResponseDTO {
    pub event_id: uuid::Uuid,
}
