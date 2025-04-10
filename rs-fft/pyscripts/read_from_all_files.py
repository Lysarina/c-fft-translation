import numpy as np
import os
from scipy import stats
import matplotlib.pyplot as plt

runs = 40

versions = ("c", "min-mod", "unsafe", "safer-return", "safer-input-and-return", "interfaced" )
# versions = ("safer-input-and-return", "volatile-safer-input-and-return" )
methods = ("naive", "cooley-tukey", "good-thomas")

data = {}

for i in range(len(versions)):
    data[i] = {}
    for r in range(len(methods)):
        data[i][r] = np.fromfile(f"data/{versions[i]}-{methods[r]}-output.bin", dtype=np.float64).reshape((runs))
        print(data[i][r])

confidence = 0.95

for i in range(len(versions)):
    print(f"{versions[i]}\n")
    for r in range(len(methods)):
        print(f"\t{methods[r]}\n")

        mean = np.mean(data[i][r])
        print(f"\t\tMean: {mean}\n")
        sem = stats.sem(data[i][r])  # Standard error of the mean

        margin = sem * stats.t.ppf((1 + confidence) / 2.0, runs - 1)
        lower_bound = mean - margin
        upper_bound = mean + margin
        print(f"\t\t{confidence*100:.1f}% confidence interval: ({lower_bound:.5f}, {upper_bound:.5f})")

        plt.figure()
        plt.plot(data[i][r], marker='o')
        plt.title(f'Run times: {versions[i]}-{methods[r]}')
        plt.xlabel('Run')
        plt.ylabel('Time (s)')

for r in range(len(methods)):
    plt.figure(figsize=(10, 6))
    plt.title(f"Confidence Intervals for {methods[r]} Method", fontsize=14)
    plt.xlabel('Version', fontsize=12)
    plt.ylabel('Time (s)', fontsize=12)
    plt.legend(title="Versions")

    # Plot each version's confidence interval
    for i in range(len(versions)):
        mean = np.mean(data[i][r])
        sem = stats.sem(data[i][r])  # Standard error of the mean
        margin = sem * stats.t.ppf((1 + confidence) / 2.0, runs - 1)
        lower_bound = mean - margin
        upper_bound = mean + margin
        
        # Plotting the confidence interval as a shaded area
        plt.fill_between([i - 0.1, i + 0.1], lower_bound, upper_bound, color="lightgray", alpha=0.5)
        
        # Plot the mean as a point
        plt.plot(i, mean, marker='o', markersize=8, label=versions[i] if r == 0 else "")
        # print(f"{versions[i]} - {methods[r]}: ({lower_bound:.5f}, {upper_bound:.5f})")

    # Add legend only for the first plot
    # if r == 0:
    
    # Set xticks to be the version indices
    plt.xticks(range(len(versions)), versions, rotation=45)

plt.tight_layout()

plt.show()