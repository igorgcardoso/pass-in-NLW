from typing import Dict, Optional
from sqlalchemy.exc import IntegrityError, NoResultFound
from src.models.settings.connection import DBConnectionHandler
from src.models.entities.events import Event
from src.models.entities.attendees import Attendee
from src.errors.errors_types import HttpConflictError, HttpNotFoundError

class EventsRepository:
    def __init__(self, connection_handler: DBConnectionHandler):
        self.connection_handler = connection_handler

    def insert_event(self, events_info: Dict) -> Dict:
        with self.connection_handler as database:
            try:
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
            except IntegrityError:
                database.session.rollback()
                raise HttpConflictError("Event already exists")
            except Exception as e:
                database.session.rollback()
                raise e

    def get_event_by_id(self, event_id: str) -> Optional[Event]:
        with self.connection_handler as database:
            try:
                return database.session.query(Event).filter_by(id=event_id).one()
            except NoResultFound:
                return None

    def count_event_attendees(self, event_id: str) -> Dict:
        with self.connection_handler as database:

            try:
                event_maximum_attendees = (
                    database.session
                    .query(Event)
                    .filter(Event.id == event_id)
                    .with_entities(Event.maximum_attendees)
                    .one()
                )

                event_count = (
                database.session
                        .query(Event)
                        .join(Attendee, Event.id == Attendee.event_id)
                        .filter(Event.id == event_id)
                        .with_entities(
                            Attendee.id
                        )
                        .all()
                )

                return {
                    "maximumAttendees": event_maximum_attendees[0],
                    "attendeesAmount": len(event_count),
                }
            except NoResultFound:
                raise HttpNotFoundError("Event not found")
