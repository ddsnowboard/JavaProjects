from sys import stdout
import numpy as np
from sklearn.cluster import KMeans
from sklearn.decomposition import PCA
import pandas as pd

datafile = "votes_by_member.csv"

data = pd.read_csv(datafile, header=None)
complete_cluster = KMeans(n_clusters=5)
complete_cluster.fit(data)
cluster_idxs = complete_cluster.predict(data)

pca = PCA(n_components=4)
decomposed = pca.fit_transform(data)

combined = np.c_[decomposed, cluster_idxs]
np.savetxt(stdout, combined, delimiter=",", fmt="%.5f")
