use axum::{
    debug_handler,
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use uuid::Uuid;

use crate::{
    domain::Attendee, error::AppError, services::ServiceTrait, state::AppState,
    RegisterForEventRequestDTO, RegisterForEventResponseDTO,
};

#[debug_handler]
#[utoipa::path(
    post,
    tag = "attendees",
    path = "/events/{event_id}/attendees",
    params(("event_id" = Uuid, Path, description = "Event ID")),
    request_body = RegisterForEventRequestDTO,
    responses((status = CREATED, body = RegisterForEventResponseDTO), (status = BAD_REQUEST, body = ErrorResponse), (status = NOT_FOUND, body = ErrorResponse))
)]
/// Register an attendee
pub async fn register_for_event(
    state: State<AppState>,
    Path(event_id): Path<Uuid>,
    Json(body): Json<RegisterForEventRequestDTO>,
) -> Result<(StatusCode, Json<RegisterForEventResponseDTO>), AppError> {
    let event = state.services.event.get(&event_id).await?;
    let attendee = Attendee::new(None, body.name, body.email, event, None, None);
    let attendee = state.services.attendee.create(&attendee).await?;

    Ok((
        StatusCode::CREATED,
        Json(RegisterForEventResponseDTO {
            attendee_id: attendee.id(),
        }),
    ))
}
