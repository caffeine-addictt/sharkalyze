# importing required libraries

import numpy as np
import pandas as pd
import matplotlib.pyplot as plt
import seaborn as sns
from sklearn import metrics
from sklearn.model_selection import train_test_split
import warnings

warnings.filterwarnings("ignore")

# convert json file to csv file
json_file_path = ""
df = pd.read_json(json_file_path)
csv_file_path = "data.csv"
df.to_csv(csv_file_path, index=False)

data = pd.read_csv("phishing.csv")
data.head()

# Shape of dataframe
data.shape()

# Information about the dataset
data.info()

# nunique value in columns
data.nunique()

# droping index column
data = data.drop(["Index"], axis=1)

# description of dataset
data.describe()

# Splitting the dataset into dependant and independant fetature

X = data.drop(["class"], axis=1)
y = data["class"]

# Splitting the dataset into train and test sets: 80-20 split
X_train, X_test, y_train, y_test = train_test_split(
    X, y, test_size=0.2, random_state=42
)
X_train.shape, y_train.shape, X_test.shape, y_test.shape

# Creating holders to store the model performance results
ML_Model = []
accuracy = []
f1_score = []
recall = []
precision = []


# function to call for storing the results
def storeResults(model, a, b, c, d):
    ML_Model.append(model)
    accuracy.append(round(a, 3))
    f1_score.append(round(b, 3))
    recall.append(round(c, 3))
    precision.append(round(d, 3))
