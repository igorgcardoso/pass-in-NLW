from typing import Optional
from rest_framework import serializers
from rest_framework.exceptions import ValidationError
from core.models import Event


class EventSerializer(serializers.ModelSerializer):
    class Meta:
        model = Event
        fields = '__all__'
        read_only_fields = ['id', 'slug']

    def validate_title(self, value: str):
        if not value:
            raise ValidationError('Title is required')
        if len(value.strip()) < 4:
            raise ValidationError('Title must be at least 4 characters long')
        return value

    def validate_maximum_attendees(self, value: Optional[int]):
        if value is not None and value < 1:
            raise ValidationError('Maximum attendees must be at least 1')
        return value
