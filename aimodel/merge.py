import pandas as pd

# Read the two CSV files
df1 = pd.read_csv("data.csv")
df2 = pd.read_csv("good.csv")

# Concatenate the DataFrames
combined_df = pd.concat([df1, df2], ignore_index=True)

# Optional: Save the combined DataFrame to a new CSV file
combined_df.to_csv("phishing.csv", index=False)

# Display the combined DataFrame (optional)
print(combined_df)
