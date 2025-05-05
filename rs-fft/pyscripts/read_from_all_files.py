import numpy as np
import os
from scipy import stats
import matplotlib.pyplot as plt

runs = 40

versions = ["c", "min-mod", "unsafe", "safer-return", "safer-input-and-return", "interfaced"]
# versions = ("safer-input-and-return", "undisturbed-safer-input-and-return" )
methods = ("Naive", "Cooley-Tukey", "Good-Thomas")
# methods = ("good-thomas",)

data = {}

for i in range(len(versions)):
    data[i] = {}
    for r in range(len(methods)):
        data[i][r] = np.fromfile(f"data/{versions[i]}-{methods[r].lower()}-output.bin", dtype=np.float64).reshape((runs))
        print(data[i][r])

confidence = 0.95

# for i in range(len(versions)):
#     print(f"{versions[i]}\n")
#     # if (versions[i] == "interfaced"): 
#     #         data[i][r] = np.delete(data[i][r], 26)
#     for r in range(len(methods)):
#         print(f"\t{methods[r]}\n")

#         mean = np.mean(data[i][r])
#         print(f"\t\tMean: {mean}\n")
#         sem = stats.sem(data[i][r])  # Standard error of the mean

#         margin = sem * stats.t.ppf((1 + confidence) / 2.0, runs - 1)
#         lower_bound = mean - margin
#         upper_bound = mean + margin
#         print(f"\t\t{confidence*100:.1f}% confidence interval: ({lower_bound:.5f}, {upper_bound:.5f})")

#         plt.figure()
#         plt.plot(data[i][r], marker='o')
#         plt.title(f'Run times: {versions[i]}-{methods[r]}')
#         plt.xlabel('Run')
#         plt.ylabel('Time (s)')

plot_side_by_side = False

versions[0] = "original-c"
versions[2] = "c-style"

for r in range(len(methods)):
    plt.figure(figsize=(10, 6))
    if plot_side_by_side:
        plt.suptitle(f"{methods[r]}", fontsize=14)
    else: 
        plt.title(f"Confidence Intervals for {methods[r]} FFT", fontsize=14)
        plt.xlabel('Version', fontsize=12)
        plt.ylabel('Time (s)', fontsize=12)

    # Plot each version's confidence interval
    for i in range(len(versions)):
        mean = np.mean(data[i][r])
        sem = stats.sem(data[i][r])  # Standard error of the mean
        margin = sem * stats.t.ppf((1 + confidence) / 2.0, runs - 1)
        lower_bound = mean - margin
        upper_bound = mean + margin
        
        if (plot_side_by_side):
            plt.subplot(3,3,i+1)
            plt.plot(data[i][r], marker='o')
            plt.title(f"Run times for {versions[i]}")
            plt.xlabel("Run")
            plt.ylabel("Time (s)")
            plt.subplot(3,3,7)
        plt.errorbar(i, mean, yerr=margin, fmt='o', capsize=5, label=versions[i])
        # Plotting the confidence interval as a shaded area
        # plt.fill_between([i - 0.1, i + 0.1], lower_bound, upper_bound, color="lightgray", alpha=0.5)
        
        # Plot the mean as a point
        # plt.plot(i, mean, marker='o', markersize=8, label=versions[i] ) #if r == 0 else ""
        # print(f"{versions[i]} - {methods[r]}: ({lower_bound:.5f}, {upper_bound:.5f})")
    
    if (plot_side_by_side):
        plt.subplot(3,3,7)
        plt.title("Confidence Intervals")
        plt.xlabel("Version")
        plt.ylabel("Time (s)")
    else:
        plt.legend( title="Versions")
    # Set xticks to be the version indices
    plt.xticks(range(len(versions)), versions) #, rotation=45
    plt.savefig(f"figs/{methods[r].lower()}-conf-intervals.png")

plt.tight_layout()

plt.show()