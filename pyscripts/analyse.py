import numpy as np
import os
from scipy import stats
import matplotlib.pyplot as plt
import pandas as pd
import scikit_posthocs as sp
import seaborn as sns
import matplotlib.lines as mlines
import matplotlib.patches as mpatches

print_details = False
plot_independently = False # plot each individual run separately

runs = 40

confidence = 0.95
alpha = 0.05

plt.rcParams["axes.prop_cycle"] = plt.cycler('color', ["#004777", "#52243C", "#a30000","#ff7700","#efd28d","#00afb5", "#30011E", "#51ae56"])

custom_colors = ["#004777","#a30000","#ff7700","#efd28d","#00afb5"]

versions = ["c", "min-mod", "rustlike"]
prettified_versions = ["Original", "MinMod", "RustLike"]
methods = ("Naive", "Cooley-Tukey", "Good-Thomas")

data = {}
data_methods = [{}, {}] # 0 = all, 1 = original vs minmod

for i in range(len(versions)):
    data[i] = {}
    for r in range(len(methods)):
        if i == 0: 
            for k in range(2):
                data_methods[k][r] = []
        data[i][r] = np.fromfile(f"data/{versions[i]}-{methods[r].lower()}-output.bin", dtype=np.float64).reshape((runs))
        data_methods[0][r].append(data[i][r])
        if i < 2:
            data_methods[1][r].append(data[i][r])
if print_details: print(data_methods)

## PLOT ALL VERSION-METHODS SEPARATELY

if plot_independently:
    for i in range(len(versions)):
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
            plt.plot(data[i][r], marker='o', color=custom_colors[0])
            plt.xlabel('Run')
            plt.ylabel('Time (s)')
            plt.savefig(f"figs/c-fft-{methods[r]}-{versions[i]}.png")

# Reformat version names
versions = prettified_versions

## PLOT CONFIDENCE INTERVALS

line_styles = ['-', '--', ':']
markers = ['o', 's', '^']
offset = 0.2

x = np.arange(len(methods))
plt.figure(figsize=(4, 5))
plt.xlabel('Method', fontsize=12)
plt.ylabel('Time (s)', fontsize=12)

# Use a colormap to assign colors by version
color_map = plt.get_cmap('tab10')

for r, method in enumerate(methods):
    for i, version in enumerate(versions):
        values = data[i][r]
        mean = np.mean(values)
        median = np.median(values)
        sem = stats.sem(values)
        margin = sem * stats.t.ppf((1 + confidence) / 2.0, runs - 1)

        # Offset to avoid overlap
        xpos = r + (i - (len(versions)-1)/2) * offset

        eb = plt.errorbar(
            xpos,
            mean,
            yerr=margin,
            fmt=markers[i],
            capsize=5,
            color=custom_colors[r],
            linewidth=1.5
        )
        eb[-1][0].set_linestyle(line_styles[i])
        plt.plot(
            xpos,
            median,
            marker='D',
            color=custom_colors[r]
        )

plt.xticks(x, methods)
plt.tight_layout()

method_handles = [
    mpatches.Patch(color=custom_colors[r], label=methods[r])
    for r in range(len(methods))
]

version_handles = [
    mlines.Line2D([], [], color='black', linestyle=line_styles[i], marker=markers[i], label=versions[i])
    for i in range(len(versions))
]

# Create both legends and place them side-by-side above the plot
legend1 = plt.legend(handles=method_handles, title="Methods", loc='upper center',
                    bbox_to_anchor=(0.74, 1), ncol=1, frameon=True)

legend2 = plt.legend(handles=version_handles, title="Versions", loc='upper center',
                    bbox_to_anchor=(0.8, 0.775), ncol=1, frameon=True)

# Add the first legend manually to keep both
plt.gca().add_artist(legend1)

plt.savefig(f"figs/all-conf-intervals.png")

## STAT SIGNIFICANCE

dunn_results = {}  # store Dunn test results here
significant_pairs = {}  # stores significant group pairs per test
performance_comparison = {}  # Store which version was faster per significant pair

for k in range(2):
    if k == 0:
        # Compare all versions
        compcase = "--- All versions ---"
        savestr = "c-fft-perf-comparison"
    else:
        # Compare C vs minmod
        compcase = "--- C vs Minmod ---"
        savestr = "c-fft-c-vs-minmod-perf-comparison"
        versions = ["Original", "MinMod"]

    performance_comparison = {}  # Store which version was faster per significant pair
    if print_details: print(compcase)
    for m, v in data_methods[k].items():
        if (len(versions) == 2): 
            u_stat, p_val = stats.mannwhitneyu(v[0], v[1], alternative='two-sided')
            if (p_val < alpha):
                if print_details: print(f"{methods[m]}: F = {r.statistic}, p = {p_val}")
                pairwise_faster = []  # (v_low, v_high, pval)
                if np.median(v[0]) < np.median(v[1]):
                    faster = (0, 1, p_val)  # 0 faster than 1
                else:
                    faster = (1, 0, p_val)  # 1 faster than 0
                pairwise_faster.append(faster)
                performance_comparison[m] = pairwise_faster
                if print_details:
                    print(f"{methods[m]}: Mann-Whitney")
                    for a, b, p in pairwise_faster:
                            print(f"\t{versions[a]} faster than {versions[b]}, p = {p:.8f}")
        else: 
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
                            sig_pairs.append((i, j, dunn.loc[i, j]))
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
                    for a, b, p in pairwise_faster:
                            print(f"\t{versions[a]} faster than {versions[b]}, p = {p:.8f}")


    win_matrix = np.zeros((len(versions), len(versions)), dtype=int)

    # Count wins
    for results in performance_comparison.values():
        for faster, slower, _ in results:
            win_matrix[faster, slower] += 1

    if print_details: print(win_matrix)

    # Plot heatmap
    plt.figure(figsize=(6, 5))
    sns.heatmap(win_matrix, annot=True, fmt="d", cmap="Blues",
                xticklabels=versions, yticklabels=versions)
    plt.xlabel("Slower Version")
    plt.ylabel("Faster Version")
    plt.tight_layout()
    plt.savefig(f"figs/{savestr}.png")


plt.show()