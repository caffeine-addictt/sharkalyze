from flask import current_app as app, request, jsonify


BASE_URL = "/api/v1"


@app.route(f"{BASE_URL}/healthcheck")
def v1_healthcheck():
    return {"status": 200, "message": "ok"}


# route for post to ai
@app.route(f"{BASE_URL}/qr_analyse", methods=["POST"])
def qrAnalyse():
    if request.method == "POST":
        request_data = request.get_json(force=True)
        response_data = {"message": "Processing complete", "data": request_data}
        return jsonify(response_data), 201
