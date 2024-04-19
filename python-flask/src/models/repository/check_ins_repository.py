from src.errors.errors_types import HttpConflictError
from src.models.settings.connection import DBConnectionHandler
from src.models.entities.check_ins import CheckIn
from typing import Dict, Optional
from sqlalchemy.exc import IntegrityError, NoResultFound


class CheckInRepository:
    def __init__(self, connection_handler: DBConnectionHandler):
        self.connection_handler = connection_handler

    def insert_check_in(self, attendee_id: str) -> str:
        with self.connection_handler as database:
            try:
                check_in = CheckIn(attendeeId=attendee_id)
                database.session.add(check_in)
                database.session.commit()
                return attendee_id
            except IntegrityError:
                database.session.rollback()
                raise HttpConflictError("Check-in already exists")
            except Exception as e:
                database.session.rollback()
                raise e
