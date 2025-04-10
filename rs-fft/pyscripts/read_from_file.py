import numpy as np
import os
from scipy import stats
import matplotlib.pyplot as plt

runs = 30

data = np.fromfile("output-safer-returns.bin", dtype=np.float64).reshape((3, runs))
print(data)
confidence = 0.95

methods = ("Naive", "Cooley-Tukey", "Good-Thomas")

# fig_shared, ax_shared = plt.subplots()

for i in range(3):
    print(f"{methods[i]}\n")
    # data[i] = {}
    # data[i] = [ptr[i][r] for r in range(runs)]

    mean = np.mean(data[i])
    print(f"\tMean: {mean}\n")
    sem = stats.sem(data[i])  # Standard error of the mean

    margin = sem * stats.t.ppf((1 + confidence) / 2.0, runs - 1)
    lower_bound = mean - margin
    upper_bound = mean + margin
    print(f"\t{confidence*100:.1f}% confidence interval: ({lower_bound:.5f}, {upper_bound:.5f})")

    plt.figure()
    plt.plot(data[i], marker='o')
    plt.title(f'Run times: {methods[i]}')
    # if (i == 0): 
    # elif (i == 1): plt.title('Run times: Cooley-Tukey')
    # else: plt.title('Run times: Good-Thomas')
    plt.xlabel('Run')
    plt.ylabel('Time (s)')


plt.show()