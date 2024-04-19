from flask import Blueprint, request, jsonify
from src.data.attendees_handler import AttendeesHandler
from src.errors.error_handler import handle_error
from src.http_types.http_response import HttpResponse
from src.http_types.http_request import HttpRequest

attendees_routes_bp = Blueprint("attendees_routes", __name__)

@attendees_routes_bp.route("/events/<event_id>/register", methods=["POST"])
def create_attendees(event_id: str):
    try:
        attendees_handler = AttendeesHandler()
        http_request = HttpRequest(params={"event_id": event_id}, body=request.json)

        http_response = attendees_handler.register(http_request)

        return jsonify(http_response.body), http_response.status_code
    except Exception as error:
        http_response = handle_error(error)
        return jsonify(http_response.body), http_response.status_code

@attendees_routes_bp.route("/attendees/<attendee_id>/badge", methods=["GET"])
def get_attendee_badge(attendee_id: str):
    try:
        attendees_handler = AttendeesHandler()
        http_request = HttpRequest(params={"attendee_id": attendee_id})

        http_response = attendees_handler.find_attendee_badge(http_request)

        return jsonify(http_response.body), http_response.status_code
    except Exception as error:
        http_response = handle_error(error)
        return jsonify(http_response.body), http_response.status_code

@attendees_routes_bp.route("/events/<event_id>/attendees", methods=["GET"])
def get_event_attendees(event_id: str):
    try:
        attendees_handler = AttendeesHandler()
        http_request = HttpRequest(params={"event_id": event_id})

        http_response = attendees_handler.find_attendees_from_event(http_request)

        return jsonify(http_response.body), http_response.status_code
    except Exception as error:
        http_response = handle_error(error)
        return jsonify(http_response.body), http_response.status_code
