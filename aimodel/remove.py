import pandas as pd

# Load the CSV file into a DataFrame
df = pd.read_csv("phishing.csv")

# Drop the column you want to delete
# Replace 'column_name' with the actual name of the column you want to delete
df = df.drop("parent_url", axis=1)

# Save the updated DataFrame back to a CSV file
# This overwrites the original file
df.to_csv("phishing.csv", index=False)
