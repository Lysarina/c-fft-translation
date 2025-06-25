clang -c complex.c -O3
clang -c fft.c -O3
clang -c naive-benchmark.c
clang -o c-fft-benchmark-nv naive-benchmark.o fft.o complex.o -lm
./c-fft-benchmark-nv
clang -c cooley-tukey-benchmark.c
clang -o c-fft-benchmark-ct cooley-tukey-benchmark.o fft.o complex.o -lm
./c-fft-benchmark-ct
clang -c good-thomas-benchmark.c
clang -o c-fft-benchmark-gt good-thomas-benchmark.o fft.o complex.o -lm
./c-fft-benchmark-gt