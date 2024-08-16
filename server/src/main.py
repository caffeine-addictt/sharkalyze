from flask import Flask


def init_app() -> Flask:
    flask_app = Flask(__name__)

    # Import routes
    with flask_app.app_context():
        import routes  # noqa: F401

    return flask_app


app = init_app()
app.run()

__all__ = ["app"]
