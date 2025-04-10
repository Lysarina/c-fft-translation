cargo build --release
clang -c benchmark.c
clang -o rs-fft-benchmark benchmark.o target/release/librs_fft.a -lm
./rs-fft-benchmark