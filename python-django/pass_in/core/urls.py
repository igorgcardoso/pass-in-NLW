from django.urls import include, path
from .views.events import router as eventRouter
from .views.attendees import router as attendeeRouter
from drf_spectacular.views import SpectacularSwaggerView, SpectacularAPIView

urlpatterns = [
    path("", include(eventRouter.urls)),
    path("", include(attendeeRouter.urls)),
    path("docs", SpectacularSwaggerView.as_view(url_name="schema")),
    path("schema", SpectacularAPIView.as_view(), name="schema"),
]
