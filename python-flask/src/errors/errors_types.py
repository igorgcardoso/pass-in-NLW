class HttpConflictError(Exception):
    def __init__(self, message: str):
        super().__init__(message)
        self.message = message
        self.name = "Conflict"
        self.status_code = 409

class HttpNotFoundError(Exception):
    def __init__(self, message: str):
        super().__init__(message)
        self.message = message
        self.name = "Not Found"
        self.status_code = 404
