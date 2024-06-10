use axum::{
    debug_handler,
    extract::{Path, State},
    http::StatusCode,
    Json,
};

use crate::{error::AppError, state::AppState};

#[debug_handler]
#[utoipa::path(
    get,
    tag = "check-ins",
    path = "/attendees/{attendee_id}/check-in",
    params(("attendee_id" = i32, Path, description = "Attendee ID")),
    responses((status = NO_CONTENT, body = ()), (status = NOT_FOUND, body = ErrorResponse))
)]
/// Check in an attendee
pub async fn check_in(
    state: State<AppState>,
    Path(attendee_id): Path<i32>,
) -> Result<(StatusCode, Json<()>), AppError> {
    state.services.attendee.check_in(attendee_id).await?;

    Ok((StatusCode::NO_CONTENT, Json(())))
}
