#include "fft.h"
#include <stdio.h>
#include <stdlib.h>

#include <sys/time.h>

double get_time_diff(struct timeval *start, struct timeval *end) {
    return (end->tv_sec - start->tv_sec) + (end->tv_usec - start->tv_usec) / 1e6;
}

int main(void) {
    struct timeval tvBegin, tvEnd, tvDiff;
    int iter = 75000;
    int warmups = 10;
    int runs = 40;

    complex * input = (complex*) malloc(sizeof(struct complex_t) * 30);
    complex * result;
    
    /* Init inputs */
    for (int i=0; i < 30; i++) {
        input[i].re = (double) i;
        input[i].im = 0.0;
    }

    double time;

    double total_time = 0.0;

    double* vals = malloc(runs*sizeof(double));

    for (int r = 0; r < runs + warmups; r++) {
        /* Naive DFT */
        gettimeofday(&tvBegin, NULL);
        for (int i=0; i < iter; i++) {
            result = FFT_CooleyTukey(input, 30, 6, 5);
        }
        gettimeofday(&tvEnd, NULL);
        time = get_time_diff(&tvBegin, &tvEnd);
        if (r >= warmups) {
            
            total_time += time;
            vals[r-warmups] = time;
            
        }
    }
    FILE *f = fopen("cooley-tukey-output.bin", "wb");
    fwrite(vals, sizeof(double), runs, f);
    fclose(f);

    printf("Cooley-Tukey (Avg over %d runs): \t %f sec\n", runs, total_time / runs);
    free(input);
    free(result);
    free(vals);

    return 0;
}

