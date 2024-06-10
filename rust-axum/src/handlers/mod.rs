pub mod check_in;
pub mod create_event;
pub mod get_attendee_badge;
pub mod get_attendees;
pub mod get_event;
pub mod register_for_event;

use axum::{
    routing::{get, post},
    Router,
};

use crate::state::AppState;

pub fn create_router() -> Router<AppState> {
    Router::<AppState>::new()
        .route("/events", post(create_event::create_event))
        .route("/events/:event_id", get(get_event::get_event))
        .route(
            "/events/:event_id/attendees",
            post(register_for_event::register_for_event).get(get_attendees::get_attendees),
        )
        .route(
            "/attendees/:attendee_id/badge",
            get(get_attendee_badge::get_attendee_badge),
        )
        .route("/attendees/:attendee_id/check-in", get(check_in::check_in))
}
