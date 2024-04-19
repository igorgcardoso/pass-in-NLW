import uuid
from src.models.repository.check_ins_repository import CheckInRepository
from src.models.settings.connection import DBConnectionHandler
from src.models.repository.events_repository import EventsRepository
from src.models.repository.attendees_repository import AttendeesRepository
from src.http_types.http_request import HttpRequest
from src.http_types.http_response import HttpResponse


class CheckInHandler:
    def __init__(self):
        db_connection_handler = DBConnectionHandler()
        self.__check_in_repository = CheckInRepository(db_connection_handler)
        self.__events_repository = EventsRepository(db_connection_handler)
        self.__attendees_repository = AttendeesRepository(db_connection_handler)

    def registry(self, http_request: HttpRequest) -> HttpResponse:
        check_in_infos = http_request.params.get("attendee_id", "")

        self.__check_in_repository.insert_check_in(check_in_infos)

        return HttpResponse(None, 201)
