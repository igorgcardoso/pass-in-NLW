from rest_framework.exceptions import APIException

class BadRequest(APIException):
    status_code = 400
    default_code = 'bad_request'
    default_detail = 'Bad request'


    def __init__(self, detail=None):
        self.detail = detail or self.default_detail
