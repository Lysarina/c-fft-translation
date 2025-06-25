# Translations of C-FFT

Translation of the [C-FFT](https://github.com/jtfell/c-fft) project to Rust. Based on commit 9b78802.

## Original

To run original C code, in this folder run:

```
bash compile-test.sh
```
to run test

```
bash fraction-benchmark-compile.sh
```
to run benchmarks and get output

## MinMod and RustLike

In rs-fft, copy contents of translation-minmod for MinMod, or translation-rustlike for RustLike into src. Run

```
bash compile-test.sh
```
to run test

```
bash fraction-benchmark-compile.sh
```
to run benchmarks and get output

## Analysis

In pyscripts, run
```
python3 analyse.py
```
to plot results of benchmarks.

To analyse new benchmarks, add .bin files from benchmark runs to data. Format of the name should be \<version\>-\<method\>. Thereafter add the version name in `versions` and a prettified name in `prettified_versions` in `analyse.py`.