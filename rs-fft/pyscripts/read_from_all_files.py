import numpy as np
import os
from scipy import stats
import matplotlib.pyplot as plt
import pandas as pd
import scikit_posthocs as sp
import seaborn as sns
import matplotlib.lines as mlines
import matplotlib.patches as mpatches

print_details = True
plot_side_by_side = False # plot associated runs with conf intervals
plot_independently = False # plot each individual run separately
remove_spikes = False
three_versions = True

runs = 40

confidence = 0.95
alpha = 0.05

plt.rcParams["axes.prop_cycle"] = plt.cycler('color', ["#004777", "#52243C", "#a30000","#ff7700","#efd28d","#00afb5", "#30011E", "#51ae56"])

custom_colors = ["#004777","#a30000","#ff7700","#efd28d","#00afb5"]

versions = ["c", "min-mod", "unsafe", "safer-return", "safer-input-and-return", "interfaced"]
if three_versions: versions = ["c", "min-mod", "safer-input-and-return"]
# versions = ("safer-input-and-return", "undisturbed-safer-input-and-return" )
methods = ("Naive", "Cooley-Tukey", "Good-Thomas")
# methods = ("good-thomas",)

data = {}
data_methods = [{}, {}, {}] # 0 = all, 1 = rustlike, 2 = orig c vs minmod
# for k in range(3):
#     data_methods.append([])
# data_methods_c_vs_minmod = {}
# data_methods_rustlike = {}

for i in range(len(versions)):
    data[i] = {}
    for r in range(len(methods)):
        if i == 0: 
            for k in range(3):
                data_methods[k][r] = []
            # data_methods[r] = []
            # data_methods_rustlike[r] = []
            # data_methods_c_vs_minmod[r] = []
        data[i][r] = np.fromfile(f"data/{versions[i]}-{methods[r].lower()}-output.bin", dtype=np.float64).reshape((runs))
        data_methods[0][r].append(data[i][r])
        # if i > 1: data_methods_rustlike[r].append(data[i][r])
        # else: data_methods_c_vs_minmod[r].append(data[i][r])
        if i < 2:
            data_methods[2][r].append(data[i][r])
        elif not three_versions: data_methods[1][r].append(data[i][r])
if print_details: print(data_methods)

## PLOT ALL VERSION-METHODS SEPARATELY

versions = ["original-c", "min-mod", "c-style", "safer-return", "safer-input-return", "interfaced"]
if three_versions: versions = ["original-c", "min-mod", "safer-input-return"]
# spikes = { "Naive": [], "Good-Thomas": ["c-style", "interfaced"], "Cooley-Tukey": ["safer-input-return", "original-c", "safer-return"]}
spikes = { "Naive": [], "Cooley-Tukey": ["original-c", "safer-return", "safer-input-return"], "Good-Thomas": ["c-style", "interfaced"]}

# spikeidx = [34, 26, 23, 36, 39]
spikeidx = [36, 39, 23, 34, 26]
idx = 0
if remove_spikes:
    for r in range(len(methods)):
        # print(methods[r])
        for i in range(len(versions)):
            # print(versions[i])
            if (versions[i] in spikes[methods[r]]):
                new_arr = np.delete(data[i][r], spikeidx[idx])
                data[i][r] = new_arr
                data_methods[0][r][i] = new_arr
                if i > 1:
                    data_methods[1][r][i-2] = new_arr
                else:
                    data_methods[2][r][i] = new_arr
                idx += 1


# for m, v in spikes.items():
#     data[i][r] = np.delete(data[i][r], 26)

plot_only = {"Naive": [], "Cooley-Tukey": ["safer-input-return"], "Good-Thomas": ["safer-return"]}

if plot_independently:
    for i in range(len(versions)):
        # print(f"{versions[i]}\n")
        # if (versions[i] == "interfaced"): 
        #         data[i][r] = np.delete(data[i][r], 26)
        for r in range(len(methods)):
            if (versions[i] in plot_only[methods[r]]):
                print((versions[i], methods[r]))
            else: continue
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
            # plt.title(f'Run times: {versions[i]}-{methods[r]}')
            plt.xlabel('Run')
            plt.ylabel('Time (s)')
            plt.savefig(f"figs/c-fft-{methods[r]}-{versions[i]}.png")

versions = ["Original C", "Min-mod", "C-style", "Safer return", "Safer input & return", "Interfaced"]
if three_versions: versions = ["Original C", "Min-mod", "Rust-like"]


## PLOT CONFIDENCE INTERVALS

if three_versions:
    line_styles = ['-', '--', ':']
    markers = ['o', 's', '^']
    offset = 0.2

    # x = np.arange(len(versions))  # base x positions
    x = np.arange(len(methods))
    plt.figure(figsize=(10, 6))
    plt.xlabel('Version', fontsize=12)
    plt.ylabel('Time (s)', fontsize=12)

    # Use a colormap to assign colors by version
    color_map = plt.get_cmap('tab10')

    # for i, version in enumerate(versions):
    #     for r, method in enumerate(methods):
    for r, method in enumerate(methods):
        for i, version in enumerate(versions):
            values = data[i][r]
            mean = np.mean(values)
            median = np.median(values)
            sem = stats.sem(values)
            margin = sem * stats.t.ppf((1 + confidence) / 2.0, runs - 1)

            # Offset to avoid overlap
            # xpos = i + (r - (len(methods)-1)/2) * offset
            xpos = r + (i - (len(versions)-1)/2) * offset

            # Plot error bar with unique linestyle for method, color for version
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
            # Median as a diamond, same color as error bar
            plt.plot(
                xpos,
                median,
                marker='D',
                color=custom_colors[r]
            )

    # Axes & ticks
    # plt.xticks(x, versions)
    plt.xticks(x, methods)
    plt.tight_layout()

    method_handles = [
        #mlines.Line2D([], [], color='black', linestyle=line_styles[r], marker=markers[r], label=methods[r])
        mpatches.Patch(color=custom_colors[r], label=methods[r])
        for r in range(len(methods))
    ]

    # Legend for versions (colors)
    version_handles = [
        #mpatches.Patch(color=custom_colors[i], label=versions[i])
        mlines.Line2D([], [], color='black', linestyle=line_styles[i], marker=markers[i], label=versions[i])
        for i in range(len(versions))
    ]

    # Create both legends and place them side-by-side above the plot
    legend1 = plt.legend(handles=method_handles, title="Methods", loc='upper center',
                        bbox_to_anchor=(0.77, 1), ncol=1, frameon=True)

    legend2 = plt.legend(handles=version_handles, title="Versions", loc='upper center',
                        bbox_to_anchor=(.92, 1), ncol=1, frameon=True)

    # Add the first legend manually to keep both
    plt.gca().add_artist(legend1)

    #plt.legend(handles=version_handles, title="Versions", loc='upper right')

    plt.savefig(f"figs/all-3m-conf-intervals.png")
else: 
    for r in range(len(methods)):
        plt.figure(figsize=(10, 6))
        if plot_side_by_side:
            plt.suptitle(f"{methods[r]}", fontsize=14)
        else: 
            plt.xlabel('Version', fontsize=12)
            plt.ylabel('Time (s)', fontsize=12)

        # Plot each version's confidence interval
        for i in range(len(versions)):
            # if i < 2: continue
            # print(f"{methods[r]}: {versions[i]}")
            
            mean = np.mean(data[i][r])
            median = np.median(data[i][r])
            sem = stats.sem(data[i][r])  # Standard error of the mean
            margin = sem * stats.t.ppf((1 + confidence) / 2.0, runs - 1)
            lower_bound = mean - margin
            upper_bound = mean + margin
            
            if (plot_side_by_side):
                #plt.subplot(3,2,i+1)
                plt.subplot(3,3,i+1)
                plt.plot(data[i][r], marker='o')
                plt.title(f"{versions[i]}")
                plt.xlabel("Run")
                plt.ylabel("Time (s)")
                plt.subplot(3,3,len(versions)+1)
            plt.errorbar(i, mean, yerr=margin, fmt='o', capsize=5, label=versions[i])
            plt.plot(i, median, marker='D', color=plt.gca().lines[-1].get_color())
        
        if (plot_side_by_side):
            plt.subplot(3,3,len(versions)+1)
            plt.title("Confidence Intervals")
            plt.xlabel("Version")
            plt.ylabel("Time (s)")
        else:
            plt.legend( title="Versions")
        # Set xticks to be the version indices
        plt.xticks(range(len(versions)), versions) #, rotation=45
        plt.tight_layout()
        plt.savefig(f"figs/{methods[r].lower()}-conf-intervals.png")
        # plt.savefig(f"figs/{methods[r].lower()}-all.png")



## STAT SIGNIFICANCE

dunn_results = {}  # store Dunn test results here
significant_pairs = {}  # stores significant group pairs per test
performance_comparison = {}  # Store which version was faster per significant pair

for k in range(3):
    if k == 0:
        # Compare all versions
        compcase = "--- All versions ---"
        savestr = "c-fft-perf-comparison"
        versions = ["Original C", "Min-mod", "C-style", "Safer return", "Safer input\n& return", "Interfaced"]
        if three_versions: 
            versions = ["Original C", "Min-mod", "Rust-like"]
            savestr = "c-fft-3m-perf-comparison"
    elif k == 1:
        if three_versions: continue
        # Compare the Rustlike versions
        compcase = "--- Rustlike versions ---"
        savestr = "c-fft-rustlike-perf-comparison"
        versions = ["C-style", "Safer return", "Safer input\n& return", "Interfaced"]
    else:
        # Compare C vs minmod
        compcase = "--- C vs Minmod ---"
        savestr = "c-fft-c-vs-minmod-perf-comparison"
        versions = ["Original C", "Min-mod"]

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
                    # print("Significant pairwise differences (p < 0.05):")
                    # for pair in sig_pairs:
                    #     print(f"\t{versions[pair[0]]} vs {versions[pair[1]]}: p = {pair[2]:.8f}")
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