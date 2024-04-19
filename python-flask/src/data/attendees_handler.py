import uuid
from src.errors.errors_types import HttpConflictError, HttpNotFoundError
from src.http_types.http_request import HttpRequest
from src.http_types.http_response import HttpResponse
from src.models.repository.attendees_repository import AttendeesRepository
from src.models.settings.connection import DBConnectionHandler
from src.models.repository.events_repository import EventsRepository


class AttendeesHandler:
    def __init__(self):
        db_connection_handler = DBConnectionHandler()
        self.__attendees_repository = AttendeesRepository(db_connection_handler)
        self.__events_repository = EventsRepository(db_connection_handler)

    def register(self, http_request: HttpRequest) -> HttpResponse:
        body = http_request.body if http_request.body else {}
        event_id = http_request.params.get("event_id")

        if event_id is None:
            raise Exception("Event ID is required")

        event_attendees_count = self.__events_repository.count_event_attendees(event_id)

        if event_attendees_count.get("maximumAttendees", 0) <= event_attendees_count.get("attendeesAmount", 0):
            raise HttpConflictError("Event is full")

        body["uuid"] = str(uuid.uuid4())
        body["event_id"] = event_id

        self.__attendees_repository.insert_attendee(body)

        return HttpResponse(None, 201)

    def find_attendee_badge(self, http_request: HttpRequest) -> HttpResponse:
        attendee_id = http_request.params.get("attendee_id")

        if not attendee_id:
            raise Exception("Attendee ID is required")

        badge = self.__attendees_repository.get_attendee_badge_by_id(attendee_id)

        if not badge:
            raise HttpNotFoundError("Attendee not registered")

        return HttpResponse(
            {
                "badge": {
                    "name": badge.name,
                    "email": badge.email,
                    "event": badge.title
                }
            }
        )

    def find_attendees_from_event(self, http_request: HttpRequest) -> HttpResponse:
        event_id = http_request.params.get("event_id")

        if not event_id:
            raise Exception("Event ID is required")

        attendees = self.__attendees_repository.get_attendees_by_event_id(event_id)

        if not attendees:
            raise HttpNotFoundError("No attendees found for this event")

        formatted_attendees = list(map(lambda attendee: {"id": attendee.id, "name": attendee.name, "email": attendee.email, "checkedInAt": attendee.checkedInAt, "registeredAt": attendee.registeredAt}, attendees))

        return HttpResponse({"attendees": formatted_attendees})
