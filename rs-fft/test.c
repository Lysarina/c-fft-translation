#include "src/rs_fft.h"
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define BUFFER_SIZE 10000

int main(void) {
    complex * input1 = (complex*) malloc(sizeof(struct complex_t) * 30);
    complex * input2 = (complex*) malloc(sizeof(struct complex_t) * 30);
    complex * result1, * result2;
    
    /* Init inputs */
    for (int i=0; i < 30; i++) {
        input1[i].re = (double) i;
        input1[i].im = 0.0;
        input2[i].re = (double) i;
        input2[i].im = 0.0;
    }
    
    /* Do FFT */
    result1 = FFT_CooleyTukey(input1, 30, 6, 5);
    result2 = FFT_GoodThomas(input2, 30, 6, 5);


    char buffer[BUFFER_SIZE] = "";  // Initialize an empty string
    
    /* Compare results */
    printf("Index \t Cooley-Tukey Output \t \t Good-Thomas Output \n");
    char *to_append = "Index \t Cooley-Tukey Output \t \t Good-Thomas Output \n";
    strcat(buffer, to_append);
    for (int i=0; i < 30; i++) {
        printf("%d: \t %f + %fi \t %f + %fi \n", i, result1[i].re, result1[i].im, 
                result2[i].re, result2[i].im);

        char line[128];
        snprintf(line, sizeof(line), "%d: \t %f + %fi \t %f + %fi \n",
                    i, result1[i].re, result1[i].im, result2[i].re, result2[i].im);

        if (strlen(buffer) + strlen(line) >= BUFFER_SIZE) {
            printf("Buffer overflow risk, stopping!\n");
            return 0;
        }
        strcat(buffer, line);
    }

    // Open the file for reading
    FILE *file = fopen("result-correct.txt", "r");
    if (!file) {
        perror("Failed to open file");
        return 0;
    }

    // Read the file contents
    char file_contents[BUFFER_SIZE] = "";
    fread(file_contents, 1, BUFFER_SIZE - 1, file);
    fclose(file);

    // Compare the generated string with the file contents
    if (strcmp(buffer, file_contents) == 0) {
        printf("Strings match!\n");
    } else {
        printf("Strings do not match.\n");
        // printf("Generated: \"%s\"\n", buffer);
        // printf("File: \"%s\"\n", file_contents);
    }
    

    free(result1);
    free(result2);
    return 0;
}

