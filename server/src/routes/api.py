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

    file_path = "server/a.txt"
    with open(file_path, "w") as file:
        file.write(request_data)

    # cargo run provided url
    try:
        url = "server/a.txt"
        proc = subprocess.run(
            ["cargo", "run", url], text=True, stdout=subprocess.PIPE, timeout=120
        )
        file_path = proc.stdout.splitlines()[2][len("Written to ") :]
        print("changed")
    except Exception as e:
        return jsonify("URL does not exist")
        print(e)

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
    df = pd.read_csv("url.csv")
    df = df.drop("url", axis=1)
    df = df.drop("parent_url", axis=1)
    df.to_csv("url.csv", index=False)

    # predict
    df = pd.read_csv("url.csv")
    print("checkpoint")
    y_pred = int(gbc_parser.predict(df)[0])
    # 1 is safe
    # -1 is unsafe
    y_pro_phishing = float(gbc_parser.predict_proba(df)[0, 0])
    y_pro_non_phishing = float(gbc_parser.predict_proba(df)[0, 1])
    print(f"y pro : {y_pro_phishing}")
    print(f"y pred : {y_pred}")
    print(f"y non pro : {y_pro_non_phishing}")
    """
    if (y_pred == 0):
        pred = y_pro_phishing * 100
        response_data = {
        "prediction": y_pred,
        "probability_phishing": round(y_pro_phishing, 2),
        "probability_non_phishing": round(y_pro_non_phishing, 2),
        "formatted_message": pred,
    }
    else:
        pred = y_pro_non_phishing * 100
        response_data = {
        "prediction": y_pred,
        "probability_phishing": round(y_pro_phishing, 2),
        "probability_non_phishing": round(y_pro_non_phishing, 2),
        "formatted_message": pred,
    }
    """

    response_data = {
        "prediction": y_pred,
        "probability_phishing": round(y_pro_phishing, 2),
        "probability_non_phishing": round(y_pro_non_phishing, 2),
    }

    # Return JSON response
    # print(pred)
    return jsonify(response_data), 201
