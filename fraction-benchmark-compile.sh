clang -c complex.c -O3
clang -c fft.c -O3
clang -c naive-benchmark.c
clang -o c-fft-benchmark naive-benchmark.o complex.o fft.o -lm
./c-fft-benchmark
clang -c cooley-tukey-benchmark.c
clang -o c-fft-benchmark cooley-tukey-benchmark.o complex.o fft.o -lm
./c-fft-benchmark
clang -c good-thomas-benchmark.c
clang -o c-fft-benchmark good-thomas-benchmark.o complex.o fft.o -lm
./c-fft-benchmark