use serde::Serialize;
use utoipa::ToSchema;

#[derive(Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct AttendeeBadgeDTO {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub event_title: String,
    pub check_in_url: String,
}

#[derive(Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct AttendeeBadgeResponseDTO {
    pub badge: AttendeeBadgeDTO,
}
