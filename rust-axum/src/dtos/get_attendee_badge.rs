use serde::Serialize;
use utoipa::ToSchema;

#[derive(Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct AttendeeBadgeDTO {
    pub name: String,
    pub email: String,
    pub event_title: String,
    pub check_in_url: String,
}
