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
