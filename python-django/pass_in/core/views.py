from django.utils.timezone import localtime
from rest_framework.decorators import api_view
from rest_framework.request import Request
from rest_framework.response import Response

from core.serializers.event_serializer import EventSerializer

# Create your views here.

@api_view(['POST'])
def create_event(request: Request) -> Response:
    serializer = EventSerializer(data=request.data)
    serializer.is_valid(raise_exception=True)
    slug = localtime().isoformat()
    serializer.save(slug=slug)
    return Response(serializer.data, status=201)
