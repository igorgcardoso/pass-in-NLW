from rest_framework.viewsets import GenericViewSet, mixins
from rest_framework.request import Request
from rest_framework.response import Response
from rest_framework.routers import SimpleRouter
from rest_framework.exceptions import APIException
from rest_framework.decorators import action
from core.serializers import EventSerializer, AttendeeSerializer
from core.utils.generate_slug import generate_slug
from core.models import Attendee, Event

router = SimpleRouter()


class AttendeesView(GenericViewSet):
    serializer_class = AttendeeSerializer
    queryset = Attendee.objects.all()

    @action(detail=True, methods=['get'], url_path="badge")
    def get_badge(self, request: Request, pk: str) -> Response:
        attendee = self.get_object()
        return Response(self.get_serializer(attendee).data)



router.register('attendees', AttendeesView)
