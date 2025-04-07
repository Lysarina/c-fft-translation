#include "src/rs_fft.h"
#include <stdio.h>
#include <stdlib.h>

#include <sys/time.h>

// void timeSubtract(struct timeval *result, struct timeval *t2, struct timeval *t1)
// {
//     long diff = (t2->tv_usec + 1000000 * t2->tv_sec) - (t1->tv_usec + 1000000 * t1->tv_sec);
//     result->tv_sec = diff / 1000000;
//     result->tv_usec = diff % 1000000;
// }

double get_time_diff(struct timeval *start, struct timeval *end) {
    return (end->tv_sec - start->tv_sec) + (end->tv_usec - start->tv_usec) / 1e6;
}

int main(void) {
    struct timeval tvBegin, tvEnd, tvDiff;
    int iter = 100000;
    int warmups = 10;
    int runs = 40;

    complex * input = (complex*) malloc(sizeof(struct complex_t) * 30);
    complex * result;
    
    /* Init inputs */
    for (int i=0; i < 30; i++) {
        input[i].re = (double) i;
        input[i].im = 0.0;
    }

    double total_time_naive = 0.0;
    double total_time_cooley_tukey = 0.0;
    double total_time_good_thomas = 0.0;

    double** vals = malloc(3*sizeof(double*));
    for (int i = 0; i < 3; i++) {
        vals[i] = (double*) malloc(sizeof(double)*runs);
    }

    for (int r = 0; r < runs; r++) {
        /* Naive DFT */
        gettimeofday(&tvBegin, NULL);
        for (int i=0; i < iter; i++) {
            result = DFT_naive(input, 30);
        }
        gettimeofday(&tvEnd, NULL);
        total_time_naive += get_time_diff(&tvBegin, &tvEnd);
        vals[0][r] = get_time_diff(&tvBegin, &tvEnd);
        // double elapsed = (tvEnd.tv_sec - tvBegin.tv_sec) + (tvEnd.tv_usec - tvBegin.tv_usec) / 1e6;
        // printf("%d x Naive: \t %f\n", iter, elapsed);

        /* Cooley-Tukey */
        gettimeofday(&tvBegin, NULL);
        for (int i=0; i < iter; i++) {
            result = FFT_CooleyTukey(input, 30, 6, 5);
        }
        gettimeofday(&tvEnd, NULL);
        total_time_cooley_tukey += get_time_diff(&tvBegin, &tvEnd);
        // vals[1][r] = get_time_diff(&tvBegin, &tvEnd);
        // elapsed = (tvEnd.tv_sec - tvBegin.tv_sec) + (tvEnd.tv_usec - tvBegin.tv_usec) / 1e6;
        // printf("%d x Cooley-Tukey: \t %f\n", iter, elapsed);

        /* Good-Thomas */
        gettimeofday(&tvBegin, NULL);
        for (int i=0; i < iter; i++) {
            result = FFT_GoodThomas(input, 30, 6, 5);
        }
        gettimeofday(&tvEnd, NULL);
        total_time_good_thomas += get_time_diff(&tvBegin, &tvEnd);
        // vals[2][r] = get_time_diff(&tvBegin, &tvEnd);
        // elapsed = (tvEnd.tv_sec - tvBegin.tv_sec) + (tvEnd.tv_usec - tvBegin.tv_usec) / 1e6;
        // printf("%d x Good-Thomas: \t %f\n", iter, elapsed);
    }
    
    printf("Naive DFT (Avg over %d runs): \t\t %f sec\n", runs, total_time_naive / runs);
    printf("Cooley-Tukey FFT (Avg over %d runs): \t %f sec\n", runs, total_time_cooley_tukey / runs);
    printf("Good-Thomas FFT (Avg over %d runs): \t %f sec\n", runs, total_time_good_thomas / runs);
    free(input);
    free(result);

    return 0;
}

