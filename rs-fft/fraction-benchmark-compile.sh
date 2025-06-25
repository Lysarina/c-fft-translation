cargo build --release
clang -c naive-benchmark.c
clang -o rs-fft-benchmark-nv naive-benchmark.o target/release/librs_fft.a -lm -ldl -lpthread
./rs-fft-benchmark-nv
clang -c cooley-tukey-benchmark.c
clang -o rs-fft-benchmark-ct cooley-tukey-benchmark.o target/release/librs_fft.a -lm -ldl -lpthread
./rs-fft-benchmark-ct
clang -c good-thomas-benchmark.c
clang -o rs-fft-benchmark-gt good-thomas-benchmark.o target/release/librs_fft.a -lm -ldl -lpthread
./rs-fft-benchmark-gt