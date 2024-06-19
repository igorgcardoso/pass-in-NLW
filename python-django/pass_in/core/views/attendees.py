from rest_framework import serializers
from rest_framework.views import status
from rest_framework.viewsets import GenericViewSet, mixins
from rest_framework.request import Request
from rest_framework.response import Response
from rest_framework.routers import SimpleRouter
from rest_framework.exceptions import APIException
from rest_framework.decorators import action
from core.serializers import EventSerializer, AttendeeSerializer, BadgeSerializer
from core.utils.generate_slug import generate_slug
from core.models import Attendee, CheckIn, Event
from core.errors import BadRequest
from drf_spectacular.utils import extend_schema, OpenApiParameter, OpenApiResponse
from drf_spectacular.types import OpenApiTypes

router = SimpleRouter()


class AttendeesView(GenericViewSet):
    serializer_class = AttendeeSerializer
    queryset = Attendee.objects.all()

    @extend_schema(summary="Get an attendee badge", request=None, responses={200: BadgeSerializer, 404: OpenApiResponse({"detail": "Not found."})})
    @action(detail=True, methods=['get'], url_path="badge")
    def get_badge(self, request: Request, pk: str) -> Response:
        attendee = self.get_object()

        base_uri = f"{request.scheme}://{request.get_host()}"
        check_in_url = f"{base_uri}/attendees/{attendee.id}/check-in"

        return Response({
            "badge": {
                "name": attendee.name,
                "email": attendee.email,
                "event_title": attendee.event.title,
                "check_in_url": check_in_url
            }
        })

    @extend_schema(tags=["check-ins"], summary="Check-in an attendee", responses={201: None}, request=None)
    @action(detail=True, methods=['post'], url_path="check-in")
    def check_in(self, request: Request, pk: str) -> Response:
        if CheckIn.objects.filter(attendee_id=pk).exists():
            raise BadRequest('Attendee already checked in.')

        CheckIn.objects.create(attendee_id=pk)

        return Response(status=status.HTTP_201_CREATED)



router.register('attendees', AttendeesView)
