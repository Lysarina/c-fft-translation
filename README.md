# Translations of C-FFT

Translation of the [C-FFT](https://github.com/jtfell/c-fft) project to Rust. Based on commit 9b78802.

## Original

To run original C code, in this directory run

```
bash compile-test.sh
```
to run test, or

```
bash fraction-benchmark-compile.sh
```
to run benchmarks and get output

## MinMod and RustLike

In the rs-fft directory, copy contents of translation-minmod for MinMod, or translation-rustlike for RustLike into src. Still in rs-fft, run

```
bash compile-test.sh
```
to run test, or

```
bash fraction-benchmark-compile.sh
```
to run benchmarks and get output.

## Analysis

In the pyscripts directory, run
```
python3 analyse.py
```
to plot results of benchmarks.

To analyse new benchmarks, add .bin files from benchmark runs to the data directory. Format of the name should be `<version>-<method>`. Thereafter add the version name in `variants` and a prettified name in `prettified_variants` in `analyse.py`.