import json
import numpy as np
import cvxpy as cp
from scipy.linalg import sqrtm
import argparse

# get arguments
parser = argparse.ArgumentParser()
parser.add_argument('-f', type=str, help='the file path')
args = parser.parse_args()

# get file path
file_path = args.f.strip()

with open(file_path, 'r') as f:
    
    # read the file line by line ignoring the first one
    data = f.read().strip().split('\n')[1:]
    
    # initalise empty list of edges
    edges = []
    
    # fill list with tuples (node_1,node_2)
    for line in data:
        values = line.split()
        edges.append((int(values[0]), int(values[1])))

#calculate n (number of nodes)
n = max(max(edge) for edge in edges)

#initalise symetric n*n matrix
X= cp.Variable((n,n),symmetric=True)

# matrix is positiv semi-definite
constraints = [X >> 0]

# set diagonals to one to get unit vectors
constraints += [
    X[i,i] == 1 for i in range(n)
]

# set objecttive funktion
objective = sum( 0.5*(1-X[i-1,j-1]) for (i,j) in edges )

#create problem (maximise objective)
prob = cp.Problem(cp.Maximize(objective),constraints)

#solve
prob.solve()

# take matrix squareroot
x = sqrtm(X.value) 

# normal to a random hyperplane
u = np.random.randn(n) 

# pick labels for the nodes according to wich side of the hyperplane they fall 
x = np.sign(x @ u) 

# convert to real numbers and then replace -1 with 0
result = np.where(x.real == -1, 0, x.real).astype(int).tolist()

# print it so it can be read by goemans_williamson.rs
print(json.dumps(result))
