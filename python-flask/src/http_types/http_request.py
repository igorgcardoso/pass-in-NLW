from typing import Dict, Optional


class HttpRequest:
    def __init__(self, body: Optional[Dict] = None, params: Optional[Dict] = None):
        self.body = body if body else {}
        self.params = params if params else {}
