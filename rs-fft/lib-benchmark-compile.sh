cargo build --release
clang -c benchmarklib.c
clang -shared -o rs-fft-benchmark-unsafe-no-malloc.so benchmarklib.o target/release/librs_fft.a -lm -O3
# python3 plot.py
# ./rs-fft-benchmark