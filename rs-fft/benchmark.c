#include "src/rs_fft.h"
#include <stdio.h>
#include <stdlib.h>

#include <sys/time.h>

double get_time_diff(struct timeval *start, struct timeval *end) {
    return (end->tv_sec - start->tv_sec) + (end->tv_usec - start->tv_usec) / 1e6;
}

// #include <time.h>

// double get_time_diff(struct timespec *start, struct timespec *end) {
//     return (end->tv_sec - start->tv_sec) + (end->tv_nsec - start->tv_nsec) / 1e9;
// }


int main(void) {
    struct timeval tvBegin, tvEnd, tvDiff;
    // struct timespec start, end;
    int iter = 100000;
    int warmups = 10;
    int runs = 30;

    complex * input = (complex*) malloc(sizeof(struct complex_t) * 30);
    complex * result;
    
    /* Init inputs */
    for (int i=0; i < 30; i++) {
        input[i].re = (double) i;
        input[i].im = 0.0;
    }

    double time;

    double total_time_naive = 0.0;
    double total_time_cooley_tukey = 0.0;
    double total_time_good_thomas = 0.0;

    double** vals = malloc(3*sizeof(double*));
    for (int i = 0; i < 3; i++) {
        vals[i] = (double*) malloc(sizeof(double)*runs);
    }

    for (int r = 0; r < runs + warmups; r++) {
        /* Naive DFT */
        // printf("Run %d\n", r);
        gettimeofday(&tvBegin, NULL);
        for (int i=0; i < iter; i++) {
            result = DFT_naive(input, 30);
        }
        gettimeofday(&tvEnd, NULL);
        time = get_time_diff(&tvBegin, &tvEnd);
        if (r >= warmups) {
            
            total_time_naive += time;
            vals[0][r-warmups] = time;
            
        }
        // printf("%f\n", time);

        /* Cooley-Tukey */
        gettimeofday(&tvBegin, NULL);
        for (int i=0; i < iter; i++) {
            result = FFT_CooleyTukey(input, 30, 6, 5);
        }
        gettimeofday(&tvEnd, NULL);
        time = get_time_diff(&tvBegin, &tvEnd);
        if (r >= warmups) {
            
            total_time_cooley_tukey += time;
            vals[1][r-warmups] = time;
            
        }
        // printf("%f\n", time);

        /* Good-Thomas */
        gettimeofday(&tvBegin, NULL);
        for (int i=0; i < iter; i++) {
            result = FFT_GoodThomas(input, 30, 6, 5);
        }
        gettimeofday(&tvEnd, NULL);
        time = get_time_diff(&tvBegin, &tvEnd);
        if (r >= warmups) {
            
            total_time_good_thomas += time;
            vals[2][r-warmups] = time;
           
        }
        // printf("%f\n", time);
    }
    
    FILE *f = fopen("output-safer-returns.bin", "wb");
    for (int i = 0; i < 3; i++) {
        fwrite(vals[i], sizeof(double), runs, f);
        free(vals[i]);
    }
    free(vals);
    // fwrite(vals, sizeof(double), 3*runs, f);
    fclose(f);

    printf("Naive DFT (Avg over %d runs): \t\t %f sec\n", runs, total_time_naive / runs);
    printf("Cooley-Tukey FFT (Avg over %d runs): \t %f sec\n", runs, total_time_cooley_tukey / runs);
    printf("Good-Thomas FFT (Avg over %d runs): \t %f sec\n", runs, total_time_good_thomas / runs);
    free(input);
    free(result);

    return 0;
}

