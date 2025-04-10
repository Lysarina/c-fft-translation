clang -c complex.c -O3
clang -c fft.c -O3
clang -c test.c
clang -o c-fft test.o complex.o fft.o -lm
./c-fft