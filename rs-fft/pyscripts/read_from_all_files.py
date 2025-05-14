import numpy as np
import os
from scipy import stats
import matplotlib.pyplot as plt
import pandas as pd
import scikit_posthocs as sp
import seaborn as sns

print_details = True

runs = 40

confidence = 0.95
alpha = 0.05

versions = ["c", "min-mod", "unsafe", "safer-return", "safer-input-and-return", "interfaced"]
# versions = ("safer-input-and-return", "undisturbed-safer-input-and-return" )
methods = ("Naive", "Cooley-Tukey", "Good-Thomas")
# methods = ("good-thomas",)

data = {}
data_methods = {}

for i in range(len(versions)):
    data[i] = {}
    for r in range(len(methods)):
        if i == 0: data_methods[r] = []
        data[i][r] = np.fromfile(f"data/{versions[i]}-{methods[r].lower()}-output.bin", dtype=np.float64).reshape((runs))
        data_methods[r].append(data[i][r])
if print_details: print(data_methods)

versions = ["Original C", "Min-mod", "C-style", "Safer return", "Safer input & return", "Interfaced"]

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

# versions[0] = "original-c"
# versions[2] = "c-style"

for r in range(len(methods)):
    plt.figure(figsize=(10, 6))
    if plot_side_by_side:
        plt.suptitle(f"{methods[r]}", fontsize=14)
    else: 
        # plt.title(f"Confidence Intervals for {methods[r]} FFT", fontsize=14)
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
    plt.tight_layout()
    plt.savefig(f"figs/{methods[r].lower()}-conf-intervals.png")



# plt.show()

dunn_results = {}  # store Dunn test results here
significant_pairs = {}  # stores significant group pairs per test
performance_comparison = {}  # Store which version was faster per significant pair


for m, v in data_methods.items():
# t = test name
# v = array of arrays of test results
# for t, v in data_test.items():
    r = stats.kruskal(*v)
    if (r.pvalue < alpha):
        if print_details: print(f"{methods[m]}: F = {r.statistic}, p = {r.pvalue}")

        # Flatten and prepare for Dunn test
        data = np.concatenate(v)
        groups = [i for i, arr in enumerate(v) for _ in arr]
        df = pd.DataFrame({'score': data, 'group': groups})

        # Run Dunn's test with long-form input
        dunn = sp.posthoc_dunn(df, val_col='score', group_col='group', p_adjust='bonferroni')
        dunn_results[m] = dunn

        # Extract significant pairs
        sig_pairs = []
        pairwise_faster = []  # (v_low, v_high, pval)
        for i in dunn.index:
            for j in dunn.columns:
                if i < j and dunn.loc[i, j] < alpha:
                    sig_pairs.append((i, j, dunn.loc[i, j]))  # optionally include p-value
                    # idx_i = int(i[1]) - 1  # convert 'v1' to 0
                    # idx_j = int(j[1]) - 1

                    median_i = np.median(v[i])
                    median_j = np.median(v[j])

                    if median_i < median_j:
                        faster = (i, j, dunn.loc[i, j])  # i faster than j
                    else:
                        faster = (j, i, dunn.loc[i, j])  # j faster than i

                    pairwise_faster.append(faster)
        significant_pairs[m] = sig_pairs
        performance_comparison[m] = pairwise_faster

        if print_details:
            print(f"{methods[m]}: Dunn")
            print(dunn)
            # print("Significant pairwise differences (p < 0.05):")
            # for pair in sig_pairs:
            #     print(f"\t{versions[pair[0]]} vs {versions[pair[1]]}: p = {pair[2]:.8f}")
            for a, b, p in pairwise_faster:
                    print(f"\t{versions[a]} faster than {versions[b]}, p = {p:.8f}")


win_matrix = np.zeros((6, 6), dtype=int)

# Count wins
for results in performance_comparison.values():
    for faster, slower, _ in results:
        win_matrix[faster, slower] += 1

print(win_matrix)

# Plot heatmap
plt.figure(figsize=(6, 5))
sns.heatmap(win_matrix, annot=True, fmt="d", cmap="Blues",
            xticklabels=versions, yticklabels=versions)
plt.xlabel("Slower Version")
plt.ylabel("Faster Version")
plt.tight_layout()
plt.savefig("figs/c-fft-perf-comparison.png")
plt.show()