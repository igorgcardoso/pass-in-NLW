from typing import Dict, Optional


class HttpResponse:
    def __init__(self, body: Optional[Dict], status_code: int = 200):
        self.body = body
        self.status_code = status_code
