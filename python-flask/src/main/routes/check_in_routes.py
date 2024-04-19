from flask import Blueprint, request, jsonify
from src.data.ckeck_in_handler import CheckInHandler
from src.errors.error_handler import handle_error
from src.http_types.http_request import HttpRequest


check_in_route_bp = Blueprint("check_in", __name__)

@check_in_route_bp.route("/attendees/<attendee_id>/check-in", methods=["POST"])
def create_check_in(attendee_id: str):
    try:
        check_in_handler = CheckInHandler()

        http_request = HttpRequest(params={"attendee_id": attendee_id})
        http_response = check_in_handler.registry(http_request)

        return jsonify(http_response.body), http_response.status_code
    except Exception as error:
        response = handle_error(error)
        return jsonify(response.body), response.status_code
