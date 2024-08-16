from flask import current_app as app


BASE_URL = "/api/v1"


@app.route(f"{BASE_URL}/healthcheck")
def v1_healthcheck():
    return {"status": 200, "message": "ok"}
