mod create_event;
mod get_attendee_badge;
mod get_attendees;
mod get_event;
mod register_for_event;

pub use self::{
    create_event::{CreateEventRequestDTO, CreateEventResponseDTO},
    get_attendee_badge::AttendeeBadgeDTO,
    get_attendees::{AttendeeDTO, GetAttendeesResponseDTO},
    get_event::{EventDTO, GetEventDTO},
    register_for_event::{RegisterForEventRequestDTO, RegisterForEventResponseDTO},
};
