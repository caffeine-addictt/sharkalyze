import pandas as pd
import json

# Load the JSON data
with open("output/2024-09-02_06-50-26.json", "r", encoding="utf-8") as f:
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

flat_data["class"] = 0
# Optional: Save the flattened data to a CSV file
flat_data.to_csv("good.csv", index=False)

# Display the flattened DataFrame (optional)
print(flat_data.head())
