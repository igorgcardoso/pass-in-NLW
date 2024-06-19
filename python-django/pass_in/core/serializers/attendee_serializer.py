from typing import Optional
from rest_framework import serializers
from rest_framework.exceptions import ValidationError
from core.models import Attendee


class AttendeeSerializer(serializers.ModelSerializer):
    checked_in_at = serializers.DateTimeField(read_only=True)
    class Meta:
        model = Attendee
        fields = '__all__'
        read_only_fields = ['id', 'created_at', 'event']

    def validate_name(self, value: str):
        if not value:
            raise ValidationError('Title is required')
        if len(value.strip()) < 4:
            raise ValidationError('Title must be at least 4 characters long')
        return value
