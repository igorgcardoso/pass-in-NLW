use axum::{
    debug_handler,
    extract::{Path, State},
    http::HeaderMap,
    Json,
};
use url::Url;

use crate::{error::AppError, services::ServiceTrait, state::AppState, AttendeeBadgeDTO};

#[debug_handler]
#[utoipa::path(
    get,
    tag = "attendees",
    path = "/attendees/{attendee_id}/badge",
    params(("attendee_id" = i32, Path, description = "Attendee ID")),
    responses((status = OK, body = AttendeeBadgeResponse), (status = NOT_FOUND, body = ErrorResponse))
)]
/// Get an attendee's badge
pub async fn get_attendee_badge(
    state: State<AppState>,
    Path(attendee_id): Path<i32>,
    headers: HeaderMap,
) -> Result<Json<AttendeeBadgeDTO>, AppError> {
    let attendee = state.services.attendee.get(&attendee_id).await?;

    let base_url = Url::parse(&*format!(
        "{}://{}",
        headers
            .get("x-forwarded-proto")
            .map(|v| v.to_str().unwrap_or("http"))
            .unwrap_or("http"),
        headers
            .get("x-forwarded-host")
            .map(|v| v.to_str().unwrap_or("localhost"))
            .unwrap_or("localhost")
    ))
    .unwrap();

    let check_in_url = format!(
        "{}://{}/attendees/{}/check-in",
        base_url.scheme(),
        base_url.host_str().unwrap_or("localhost"),
        attendee.id()
    );

    Ok(Json(AttendeeBadgeDTO {
        name: attendee.name().to_string(),
        email: attendee.email().to_string(),
        event_title: attendee.event().title().to_string(),
        check_in_url,
    }))
}
