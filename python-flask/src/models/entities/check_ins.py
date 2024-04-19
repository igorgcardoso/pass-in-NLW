from src.models.settings.base import Base
from sqlalchemy import Column, String, Integer, DateTime, ForeignKey
from sqlalchemy.sql import func

class CheckIn(Base):
    __tablename__ = 'check_ins'

    id = Column(Integer, primary_key=True)
    attendeeId = Column(String, ForeignKey('attendees.id'), nullable=False)
    created_at = Column(DateTime, default=func.now())

    def __repr__(self):
        return f"CheckIn [attendee_id={self.attendee_id}]"
