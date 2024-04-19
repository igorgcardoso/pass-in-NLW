from flask import Flask
from flask_cors import CORS
from src.main.routes.event_routes import event_route_bp
from src.main.routes.attendees_routes import attendees_routes_bp
from src.main.routes.check_in_routes import check_in_route_bp


app = Flask(__name__)
CORS(app)

app.register_blueprint(event_route_bp)
app.register_blueprint(attendees_routes_bp)
app.register_blueprint(check_in_route_bp)
