use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Deserialize, ToSchema)]
pub struct RegisterForEventRequestDTO {
    pub name: String,
    pub email: String,
}

#[derive(ToSchema, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RegisterForEventResponseDTO {
    pub attendee_id: i32,
}
