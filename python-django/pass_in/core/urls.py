from django.urls import include, path
from .views.events import router as eventRouter
from .views.attendees import router as attendeeRouter

urlpatterns = [
    path("", include(eventRouter.urls)),
    path("", include(attendeeRouter.urls)),
]
