import matplotlib.pyplot as plt
import numpy as np
import pandas as pd
from sys import argv

data = pd.read_csv(argv[1])

xs = data["component_1"]
ys = data["component_2"]
colors = data["party"].apply(lambda x: "red" if x == "republican" else "blue")
labels = data[["name", "party"]].apply(lambda x: "{} ({})".format(x["name"], x["party"][0].upper()), axis=1)
plt.rcParams["figure.figsize"] = (24, 10)

plt.scatter(xs, ys, c=colors)
for idx, label in enumerate(labels):
    plt.annotate(label, (xs[idx], ys[idx]))
plt.savefig(argv[2])
