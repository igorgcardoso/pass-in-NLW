use axum::{
    debug_handler,
    extract::{Path, State},
    Json,
};
use uuid::Uuid;

use crate::{error::AppError, services::ServiceTrait, state::AppState, EventDTO, GetEventDTO};

#[debug_handler]
#[utoipa::path(
    get,
    tag = "events",
    path = "/events/{event_id}",
    params(("event_id" = Uuid, Path, description = "Event ID")),
    responses((status = OK, body = GetEventDTO), (status = NOT_FOUND, body = ErrorResponse))
)]
/// Get an event
pub async fn get_event(
    state: State<AppState>,
    event_id: Path<Uuid>,
) -> Result<Json<GetEventDTO>, AppError> {
    let event = state.services.event.get(&event_id).await?;

    Ok(Json(GetEventDTO {
        event: EventDTO {
            id: event.id(),
            title: event.title().to_string(),
            details: event.details().map(|s| s.to_string()),
            slug: event.slug().to_string(),
            maximum_attendees: event.maximum_attendees(),
            attendees_amount: event.attendees_amount(),
        },
    }))
}
