from uuid import uuid4
from django.db import models

# Create your models here.
class Event(models.Model):
    id = models.CharField(max_length=36, primary_key=True, default=uuid4)
    title = models.TextField()
    details = models.TextField(null=True, blank=True)
    slug = models.SlugField(max_length=255, unique=True)
    maximum_attendees = models.PositiveIntegerField(null=True)

    class Meta:
        db_table = 'events'

    @property
    def attendees_amount(self):
        return self.attendees.count()


class Attendee(models.Model):
    id = models.AutoField(primary_key=True)
    name = models.TextField()
    email = models.EmailField()
    created_at = models.DateTimeField(auto_now_add=True)
    event = models.ForeignKey(Event, on_delete=models.CASCADE, related_name='attendees')

    class Meta:
        db_table = 'attendees'
        constraints = [
            models.UniqueConstraint(fields=['email', 'event'], name='unique_attendee')
        ]
