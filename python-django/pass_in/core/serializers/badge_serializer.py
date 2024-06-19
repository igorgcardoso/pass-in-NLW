from typing import Optional
from rest_framework import serializers
from rest_framework.exceptions import ValidationError
from core.models import Attendee

class BadgeInfoSerializer(serializers.Serializer):
    name = serializers.CharField()
    email = serializers.EmailField()
    event_title = serializers.CharField()
    check_in_url = serializers.URLField()

class BadgeSerializer(serializers.Serializer):
    badge = BadgeInfoSerializer()
