# importing required libraries

import pandas as pd
import warnings

warnings.filterwarnings("ignore")

# convert json file to csv file
json_file_path = "output/"
df = pd.read_json(json_file_path)
df["class"] = 1
csv_file_path = "data.csv"
df.to_csv(csv_file_path, index=False)
