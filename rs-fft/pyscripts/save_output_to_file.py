from cffi import FFI
import numpy as np
from scipy import stats
import matplotlib.pyplot as plt
import pickle

ffi = FFI()
lib = ffi.dlopen('./rs-fft-benchmark-interfaced.so')
ffi.cdef("""
    int get_runs();
    double** benchmark();
""")

runs = lib.get_runs()
ptr = lib.benchmark()
vals = {}

filename = "output-interfaced.txt"


for i in range(3):
    vals[i] = {}
    vals[i] = [ptr[i][r] for r in range(runs)]
with open(filename, 'wb') as f:
    pickle.dump(vals, f)

