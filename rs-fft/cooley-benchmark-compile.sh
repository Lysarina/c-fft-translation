cargo build --release
clang -c cooley-tukey-benchmark.c
clang -o rs-fft-benchmark cooley-tukey-benchmark.o target/release/librs_fft.a -lm
./rs-fft-benchmark