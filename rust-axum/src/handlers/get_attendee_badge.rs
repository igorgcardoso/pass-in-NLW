use axum::{
    debug_handler,
    extract::{Host, Path, State},
    Json,
};

use crate::{
    error::AppError, services::ServiceTrait, state::AppState, AttendeeBadgeDTO,
    AttendeeBadgeResponseDTO,
};

#[debug_handler]
#[utoipa::path(
    get,
    tag = "attendees",
    path = "/attendees/{attendee_id}/badge",
    params(("attendee_id" = i32, Path, description = "Attendee ID")),
    responses((status = OK, body = AttendeeBadgeResponseDTO), (status = NOT_FOUND, body = ErrorResponse))
)]
/// Get an attendee's badge
pub async fn get_attendee_badge(
    state: State<AppState>,
    Path(attendee_id): Path<i32>,
    host: Host,
) -> Result<Json<AttendeeBadgeResponseDTO>, AppError> {
    let attendee = state.services.attendee.get(&attendee_id).await?;

    tracing::info!("{}", host.0);

    let check_in_url = format!("{}/attendees/{}/check-in", host.0, attendee.id());

    Ok(Json(AttendeeBadgeResponseDTO {
        badge: AttendeeBadgeDTO {
            id: attendee.id(),
            name: attendee.name().to_string(),
            email: attendee.email().to_string(),
            event_title: attendee.event().title().to_string(),
            check_in_url,
        },
    }))
}
