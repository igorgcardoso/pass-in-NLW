from rest_framework.viewsets import GenericViewSet, mixins
from rest_framework.request import Request
from rest_framework.response import Response
from rest_framework.routers import SimpleRouter
from rest_framework.decorators import action
from core.serializers import EventSerializer, AttendeeSerializer
from core.utils.generate_slug import generate_slug
from core.models import Attendee, Event
from core.errors import BadRequest
from drf_spectacular.utils import extend_schema, OpenApiParameter, OpenApiResponse
from drf_spectacular.types import OpenApiTypes

router = SimpleRouter()


class EventsView(GenericViewSet, mixins.RetrieveModelMixin):
    serializer_class = EventSerializer
    queryset = Event.objects.all()

    def create(self, request: Request) -> Response:
        serializer = self.get_serializer(data=request.data)
        serializer.is_valid(raise_exception=True)

        slug = generate_slug(serializer.validated_data['title'])

        event_with_same_slug = Event.objects.filter(slug=slug).first()
        if event_with_same_slug:
            raise BadRequest('Another event with same title already exists.')

        serializer.save(slug=slug)
        return Response(serializer.data, status=201)

    @action(detail=True, methods=['get', 'post'], url_path="attendees")
    def attendees(self, request: Request, pk: str) -> Response:
        if request.method == 'GET':
            return self.get_event_attendees(request, pk)
        else:
            return self.register_for_event(request, pk)

    def register_for_event(self, request: Request, pk: str) -> Response:
        serializer = AttendeeSerializer(data=request.data)
        serializer.is_valid(raise_exception=True)

        if Attendee.objects.filter(email=serializer.validated_data['email'], event_id=pk).exists():
            raise BadRequest('This e-mail is already registered for this event.')

        event = Event.objects.get(pk=pk)

        if event.maximum_attendees is not None and event.attendees_amount >= event.maximum_attendees:
            raise BadRequest('The maximum number of attendees for this event has been reached.')

        attendee = serializer.save(event_id=pk)
        return Response({'attendee_id': attendee.id}, status=201)

    @action(detail=True, methods=['get'], url_path="attendees")
    def get_event_attendees(self, request: Request, pk: str) -> Response:
        event = self.get_object()
        attendees = event.attendees.all()

        return Response({
            "attendees": AttendeeSerializer(attendees, many=True).data
        })


router.register('events', EventsView)
