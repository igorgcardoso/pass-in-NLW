from flask import Blueprint, jsonify, request
from src.data.event_handler import EventHandler
from src.http_types.http_request import HttpRequest
from src.errors.error_handler import handle_error


event_route_bp = Blueprint('event_route', __name__)

@event_route_bp.route('/events', methods=['POST'])
def create_event():
    try:
        http_request = HttpRequest(body=request.json)
        event_handler = EventHandler()

        response = event_handler.register(http_request)
        return jsonify(response.body), response.status_code
    except Exception as error:
        response = handle_error(error)
        return jsonify(response.body), response.status_code

@event_route_bp.route('/events/<event_id>', methods=['GET'])
def get_event(event_id):
    try:
        http_request = HttpRequest(params={'event_id': event_id})
        event_handler = EventHandler()

        response = event_handler.find_by_id(http_request)
        return jsonify(response.body), response.status_code
    except Exception as error:
        response = handle_error(error)
        return jsonify(response.body), response.status_code
