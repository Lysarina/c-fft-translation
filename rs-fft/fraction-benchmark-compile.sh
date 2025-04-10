cargo build --release
clang -c naive-benchmark.c
clang -o rs-fft-benchmark naive-benchmark.o target/release/librs_fft.a -lm
./rs-fft-benchmark
clang -c cooley-tukey-benchmark.c
clang -o rs-fft-benchmark cooley-tukey-benchmark.o target/release/librs_fft.a -lm
./rs-fft-benchmark
clang -c good-thomas-benchmark.c
clang -o rs-fft-benchmark good-thomas-benchmark.o target/release/librs_fft.a -lm
./rs-fft-benchmark