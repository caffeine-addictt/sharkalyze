import pandas as pd
import warnings
from sklearn.model_selection import train_test_split
from sklearn.ensemble import GradientBoostingClassifier
from sklearn import metrics
import pickle

warnings.filterwarnings("ignore")

data = pd.read_csv("phishing.csv")

# Splitting the dataset into dependant and independant fetature
X = data.drop(["class"], axis=1)
y = data["class"]

# Splitting the dataset into train and test sets: 80-20 split
X_train, X_test, y_train, y_test = train_test_split(
    X, y, test_size=0.2, random_state=42
)

## model testing for gbc

# instantiate the model
gbc = GradientBoostingClassifier(max_depth=4, learning_rate=0.7)

# fit the model
gbc.fit(X_train, y_train)

# predicting the target value from the model for the samples
y_train_gbc = gbc.predict(X_train)
y_test_gbc = gbc.predict(X_test)

# dump information to that file
pickle.dump(gbc, open("pickle/model_parser.pkl", "wb"))

# computing the accuracy, f1_score, Recall, precision of the model performance

acc_train_gbc = metrics.accuracy_score(y_train, y_train_gbc)
acc_test_gbc = metrics.accuracy_score(y_test, y_test_gbc)
print(
    "Gradient Boosting Classifier : Accuracy on training Data: {:.3f}".format(
        acc_train_gbc
    )
)
print(
    "Gradient Boosting Classifier : Accuracy on test Data: {:.3f}".format(acc_test_gbc)
)
print()
