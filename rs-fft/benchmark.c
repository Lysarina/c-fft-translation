#include "rs_fft.h"
#include <stdio.h>
#include <stdlib.h>

#include <sys/time.h>

void timeSubtract(struct timeval *result, struct timeval *t2, struct timeval *t1)
{
    long diff = (t2->tv_usec + 1000000 * t2->tv_sec) - (t1->tv_usec + 1000000 * t1->tv_sec);
    result->tv_sec = diff / 1000000;
    result->tv_usec = diff % 1000000;
}

int main(void) {
    struct timeval tvBegin, tvEnd, tvDiff;
    int iter = 1000000;

    complex * input = (complex*) malloc(sizeof(struct complex_t) * 30);
    complex * result;
    
    /* Init inputs */
    for (int i=0; i < 30; i++) {
        input[i].re = (double) i;
        input[i].im = 0.0;
    }

    /* Naive DFT */
    gettimeofday(&tvBegin, NULL);
    for (int i=0; i < iter; i++) {
        result = DFT_naive(input, 30);
    }
    gettimeofday(&tvEnd, NULL);

    //timeSubtract(&tvDiff, &tvEnd, &tvBegin);
    //printf("%d x Naive: \t %ld.%ld\n", iter, tvDiff.tv_sec, tvDiff.tv_usec);
    double elapsed = (tvEnd.tv_sec - tvBegin.tv_sec) + (tvEnd.tv_usec - tvBegin.tv_usec) / 1e6;
    printf("%d x Naive: \t %f\n", iter, elapsed);
    
    /* Cooley-Tukey */
    gettimeofday(&tvBegin, NULL);
    for (int i=0; i < iter; i++) {
        result = FFT_CooleyTukey(input, 30, 6, 5);
    }
    gettimeofday(&tvEnd, NULL);

    // timeSubtract(&tvDiff, &tvEnd, &tvBegin);
    // printf("%d x Cooley-Tukey: \t %ld.%ld\n", iter, tvDiff.tv_sec, tvDiff.tv_usec);
    elapsed = (tvEnd.tv_sec - tvBegin.tv_sec) + (tvEnd.tv_usec - tvBegin.tv_usec) / 1e6;
    printf("%d x Cooley-Tukey: \t %f\n", iter, elapsed);

    /* Good-Thomas */
    gettimeofday(&tvBegin, NULL);
    for (int i=0; i < iter; i++) {
        result = FFT_GoodThomas(input, 30, 6, 5);
    }
    gettimeofday(&tvEnd, NULL);

    // timeSubtract(&tvDiff, &tvEnd, &tvBegin);
    // printf("%d x Good-Thomas: \t %ld.%ld\n", iter, tvDiff.tv_sec, tvDiff.tv_usec);
    elapsed = (tvEnd.tv_sec - tvBegin.tv_sec) + (tvEnd.tv_usec - tvBegin.tv_usec) / 1e6;
    printf("%d x Good-Thomas: \t %f\n", iter, elapsed);

    return 0;
}

