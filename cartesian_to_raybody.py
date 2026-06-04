#!/usr/bin/env python3
import os
import sys
import numpy as np

if len(sys.argv) != 4:
    print(f"Usage: {sys.argv[0]} input_file output_file horizon_center(In z axis)")
    sys.exit(1)
filename = sys.argv[1]
try:
    data = np.loadtxt(filename, comments="#")
except Exception as e:
    print(f"Error reading file: {e}")
    sys.exit(1)

xdata = data[:,0]
zdata = data[:,1]
midpoint = float(sys.argv[3])

r=np.sqrt(xdata**2 + (zdata-midpoint)**2)
theta=np.arctan2(xdata,zdata)

with open(sys.argv[2], "w") as f:

    f.write("# r    theta\n")

    for i in range(len(xdata)):
        f.write(f"{r[i]}\t{theta[i]}\n")

print(f"Wrote {len(xdata)} points to {sys.argv[2]}")
