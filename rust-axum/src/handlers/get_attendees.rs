use axum::{
    debug_handler,
    extract::{Path, Query, State},
    Json,
};
use serde::Deserialize;
use utoipa::ToSchema;
use uuid::Uuid;
use validator::Validate;

use crate::{error::AppError, state::AppState, AttendeeDTO, GetAttendeesResponseDTO};

#[derive(Deserialize, ToSchema, Validate)]
#[serde(rename_all = "camelCase")]
pub struct GetAttendeesQuery {
    query: Option<String>,
    #[validate(range(min = 0))]
    page_index: Option<i32>,
}

#[debug_handler]
#[utoipa::path(
    get,
    tag = "events",
    path = "/events/{event_id}/attendees",
    params(("event_id" = Uuid, Path, description = "Event ID"), ("query" = GetAttendeesQuery, Query, description = "Query parameters")),
    responses((status = OK, body = GetAttendeesResponseDTO), (status = NOT_FOUND, body = ErrorResponse))
)]
/// Get attendees
pub async fn get_attendees(
    state: State<AppState>,
    Path(event_id): Path<Uuid>,
    query: Query<GetAttendeesQuery>,
) -> Result<Json<GetAttendeesResponseDTO>, AppError> {
    let name = query.query.clone();
    let page_index = query.page_index.unwrap_or(0) as i64;

    const PAGE_SIZE: i64 = 10;

    let (attendees, total) = state
        .services
        .attendee
        .get_from_event(event_id, name, page_index, PAGE_SIZE)
        .await?;

    Ok(Json(GetAttendeesResponseDTO {
        attendees: attendees
            .iter()
            .map(|attendee| AttendeeDTO {
                id: attendee.id(),
                name: attendee.name().to_string(),
                email: attendee.email().to_string(),
                created_at: attendee.created_at(),
                checked_in_at: attendee.checked_in_at(),
            })
            .collect::<Vec<_>>(),
        total,
    }))
}
