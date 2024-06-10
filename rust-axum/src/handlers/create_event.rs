use axum::{debug_handler, extract::State, http::StatusCode, Json};

use crate::{
    domain::Event,
    dtos::{CreateEventRequestDTO, CreateEventResponseDTO},
    error::AppError,
    services::ServiceTrait,
    state::AppState,
};

#[debug_handler]
#[utoipa::path(
    post,
    tag = "events",
    path = "/events",
    request_body = CreateEventRequestDTO,
    responses((status = CREATED, body = CreateEventResponseDTO), (status = BAD_REQUEST, body = ErrorResponse), (status = UNPROCESSABLE_ENTITY, body = ErrorResponse))
)]
/// Create a new event
pub async fn create_event(
    state: State<AppState>,
    Json(body): Json<CreateEventRequestDTO>,
) -> Result<(StatusCode, Json<CreateEventResponseDTO>), AppError> {
    let event = Event::from(body);
    let event = state.services.event.create(&event).await?;

    Ok((
        StatusCode::CREATED,
        Json(CreateEventResponseDTO {
            event_id: event.id(),
        }),
    ))
}
