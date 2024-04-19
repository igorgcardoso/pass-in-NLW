from sqlalchemy.engine import Row
from src.errors.errors_types import HttpConflictError
from src.models.entities.events import Event
from src.models.settings.connection import DBConnectionHandler
from src.models.entities.attendees import Attendee
from src.models.entities.check_ins import CheckIn
from typing import Dict, List, Optional, Tuple
from sqlalchemy.exc import IntegrityError, NoResultFound


class AttendeesRepository:
    def __init__(self, connection_handler: DBConnectionHandler):
        self.connection_handler = connection_handler

    def insert_attendee(self, attendee_info: Dict) -> Dict:
        with self.connection_handler as database:
            try:
                attendee = Attendee(
                    id=attendee_info.get("uuid"),
                    name=attendee_info.get("name"),
                    email=attendee_info.get("email"),
                    event_id=attendee_info.get("event_id")
                )
                database.session.add(attendee)
                database.session.commit()
                return attendee_info
            except IntegrityError:
                database.session.rollback()
                raise HttpConflictError("Attendee already exists")
            except Exception as e:
                database.session.rollback()
                raise e

    def get_attendee_badge_by_id(self, attendee_id: str) -> Optional[Attendee]:
        with self.connection_handler as database:
            try:
                return (
                    database.session
                        .query(Attendee)
                        .join(Event, Attendee.event_id == Event.id)
                        .filter(Attendee.id == attendee_id)
                        .with_entities(Attendee.name, Attendee.email, Event.title)
                        .one()
                )
            except NoResultFound:
                return None

    def get_attendees_by_event_id(self, event_id: str) -> List[Attendee]:
        with self.connection_handler as database:
            return (
                database.session
                    .query(Attendee)
                    .outerjoin(CheckIn, Attendee.id == CheckIn.attendeeId)
                    .filter(Attendee.event_id == event_id)
                    .with_entities(
                        Attendee.id,
                        Attendee.name,
                        Attendee.email,
                        CheckIn.created_at.label("checkedInAt"),
                        Attendee.created_at.label("registeredAt")
                    )
                    .all()
            )
