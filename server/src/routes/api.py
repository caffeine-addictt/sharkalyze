from flask import current_app as app, request


BASE_URL = "/api/v1"


@app.route(f"{BASE_URL}/healthcheck")
def v1_healthcheck():
    return {"status": 200, "message": "ok"}


# route for post to ai
@app.route("/qrAnalyse", methods=["POST"])
def qrAnalyse():
    url = request.json()
    # process ai stuff here
    return {"message": "Data received successfully", "received_data": url}
