from flask import Flask
from flask_cors import CORS


def init_app() -> Flask:
    flask_app = Flask(__name__)

    # Import routes
    with flask_app.app_context():
        from . import routes  # noqa: F401

    return flask_app


app = init_app()
CORS(app)

__all__ = ["app"]
