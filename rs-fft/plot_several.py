from cffi import FFI
import numpy as np
from scipy import stats
import matplotlib.pyplot as plt

### LOADING benchmarks ###
benchmarks = {}

ffi = FFI()
lib = ffi.dlopen('./rs-fft-benchmark-unsafe.so')
ffi.cdef("""
    int get_runs();
    double** benchmark();
""")

#length = lib.get_array_length()
runs = lib.get_runs()
benchmarks[0] = lib.benchmark() # unsafe

lib = ffi.dlopen('./rs-fft-benchmark-unsafe-no-malloc.so')
ffi.cdef("""
    double** benchmark();
""")

benchmarks[1] = lib.benchmark() # unsafe, no malloc

lib = ffi.dlopen('./rs-fft-benchmark-interfaced.so')
ffi.cdef("""
    double** benchmark();
""")

benchmarks[2] = lib.benchmark() # interfaced

programs = {"Uninterfaced with unsafe return", "Uninterfaced with safe return", "Interfaced"}

### STATISTICS ###
confidence = 0.95

methods = {"Naive", "Cooley-Tukey", "Good-Thomas"}


for p in range(len(benchmarks)):
    print(f"{programs[p]}\n")
    ptr = benchmarks[p]
    vals = {}

    # fig_shared, ax_shared = plt.subplots()
    for i in range(3):
        print(f"\t{methods[i]}\n")
        vals[i] = {}
        vals[i] = [ptr[i][r] for r in range(40)]

        mean = np.mean(vals[i])
        print(f"\t\tMean: {mean}\n")
        sem = stats.sem(vals[i])  # Standard error of the mean

        margin = sem * stats.t.ppf((1 + confidence) / 2.0, runs - 1)
        lower_bound = mean - margin
        upper_bound = mean + margin
        print(f"\t\t{confidence*100:.1f}% confidence interval: ({lower_bound:.2f}, {upper_bound:.2f})")

        plt.figure()
        plt.plot(vals[i], marker='o')
        plt.title(f'Run times: {programs[p]}, {methods[i]}')
        # if (i == 0): 
        # elif (i == 1): plt.title('Run times: Cooley-Tukey')
        # else: plt.title('Run times: Good-Thomas')
        plt.xlabel('Run')
        plt.ylabel('Time (s)')


plt.show()

# print(py_array)
