from src.http_types.http_response import HttpResponse
from .errors_types import HttpConflictError, HttpNotFoundError


def handle_error(error: Exception) -> HttpResponse:
    if isinstance(error, (HttpConflictError, HttpNotFoundError)):
        return HttpResponse(
            {
                "errors": [{
                    "title": error.name,
                    "details": error.message

                }],
            },
            error.status_code,
        )

    return HttpResponse(
        {
            "errors": [{
                "title": "error",
                "details": str(error),
            }],
        },
        400,
    )
