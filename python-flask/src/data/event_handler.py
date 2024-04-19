from src.http_types.http_request import HttpRequest
from src.http_types.http_response import HttpResponse
from src.models.repository.events_repository import EventsRepository
from src.models.settings.connection import DBConnectionHandler
import uuid

class EventHandler:
    def __init__(self):
        self.__events_repository = EventsRepository(DBConnectionHandler())

    def register(self, http_request: HttpRequest) -> HttpResponse:
        body = http_request.body if http_request.body else {}
        body["uuid"] = str(uuid.uuid4())
        self.__events_repository.insert_event(body)

        return HttpResponse({"eventId": body["uuid"]}, 201)

    def find_by_id(self, http_request: HttpRequest) -> HttpResponse:
        event_id = http_request.params.get("event_id")

        if event_id is None:
            raise ValueError("Event ID is required")

        event_attendees_count = self.__events_repository.count_event_attendees(event_id)

        event = self.__events_repository.get_event_by_id(event_id)

        if event is None:
            raise Exception("Event not found")

        return HttpResponse({
            "event":
                {
                    "id": event.id,
                    "title": event.title,
                    "details": event.details,
                    "slug": event.slug,
                    "maximumAttendees": event.maximum_attendees,
                    "attendeesAmount": event_attendees_count["attendeesAmount"],
                },
            },
        200)
