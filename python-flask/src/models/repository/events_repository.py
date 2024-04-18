from typing import Dict
from src.models.settings.connection import DBConnectionHandler
from src.models.entities.events import Event

class EventsRepository:
    def __init__(self, connection_handler: DBConnectionHandler):
        self.connection_handler = connection_handler

    def insert_event(self, events_info: Dict) -> Dict:
        with self.connection_handler as database:
            event = Event(
             id=events_info.get("uuid"),
             title=events_info.get("title"),
             details=events_info.get("details"),
                slug=events_info.get("slug"),
                maximum_attendees=events_info.get("maximum_attendees")
            )
            database.session.add(event)
            database.session.commit()
            return events_info

    def get_event_by_id(self, event_id: str) -> Event:
        with self.connection_handler as database:
            return database.session.query(Event).filter_by(id=event_id).one()
