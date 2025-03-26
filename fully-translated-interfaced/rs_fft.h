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

complex *DFT_naive(complex *x, int32_t N);

complex *FFT_CooleyTukey(complex *input, int32_t N, int32_t N1, int32_t N2);

complex *FFT_GoodThomas(complex *input, int32_t N, int32_t N1, int32_t N2);

