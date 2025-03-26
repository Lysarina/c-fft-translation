cargo build --release
clang -c test.c
clang -o rs-fft test.o target/release/librs_fft.a -lm -O3
./rs-fft