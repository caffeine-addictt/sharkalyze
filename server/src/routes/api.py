from flask import current_app as app, request, jsonify, json
import pickle
import subprocess
import pandas as pd

file = open("pickle/model_parser.pkl", "rb")
gbc_parser = pickle.load(file)
file.close()


BASE_URL = "/api/v1"


@app.route(f"{BASE_URL}/healthcheck")
def v1_healthcheck():
    return {"status": 200, "message": "ok"}


# route for post to ai
@app.route(f"{BASE_URL}/qr-analyse", methods=["POST"])
def qrAnalyse():
    request_data = request.get_json(force=True)

    # store input into txt file
    file_path = "server/a.txt"
    with open(file_path, "w") as file:
        json.dump(request_data, file, indent=4)

    # cargo run provided url
    url = "server/a.txt"
    proc = subprocess.run(["cargo", "run", url], text=True, stdout=subprocess.PIPE)
    file_path = proc.stdout.splitlines()[2][len("Written to ") :]

    # convert file to csv file
    # Load the JSON data
    with open(file_path, "r", encoding="utf-8") as f:
        data = json.load(f)
    # Flatten the JSON data
    # The 'hyprlinks' field is nested, so we need to normalize it
    flat_data = pd.json_normalize(
        data,
        record_path="hyprlinks",
        meta=[
            "url",
            "is_ssl_https",
            "url_entropy",
            "is_utf8_from_header",
            "contenttype_header_contains_text_html",
        ],
        meta_prefix="parent_",
        errors="ignore",
    )
    flat_data.to_csv("url.csv", index=False)

    # remove unwanted columns
    df = pd.read_csv("phishing.csv")
    df = df.drop("url", axis=1)
    df = df.drop("parent_url", axis=1)
    df.to_csv("url.csv", index=False)

    # predict
    df = pd.read_csv("phishing.csv")
    y_pred = gbc_parser.predict(df)[0]
    # 1 is safe
    # -1 is unsafe
    y_pro_phishing = gbc_parser.predict_proba(df)[0, 0]
    y_pro_non_phishing = gbc_parser.predict_proba(df)[0, 1]
    # Format the prediction results
    pred = "It is {0:.2f}% safe to go".format(y_pro_phishing * 100)
    # Create the response data
    response_data = {
        "prediction": y_pred,
        "probability_phishing": round(y_pro_phishing, 2),
        "probability_non_phishing": round(y_pro_non_phishing, 2),
        "formatted_message": pred,
    }

    # Return JSON response
    return jsonify(response_data), 201
