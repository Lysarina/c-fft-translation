#include <stdarg.h>
#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>
#include <stdlib.h>


typedef struct complex_t {
  double re;
  double im;
} complex_t;

typedef struct complex_t complex;

complex *DFT_naive(complex *x, int N);

complex *FFT_CooleyTukey(complex *input, int N, int N1, int N2);

complex *FFT_GoodThomas(complex *input, int N, int N1, int N2);

extern complex add(complex left, complex right);

extern complex conv_from_polar(double r, double radians);

extern void free(void*);

extern void *malloc(unsigned long);

extern complex multiply(complex left, complex right);
