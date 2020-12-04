#include <stddef.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <sys/types.h>
#include <inttypes.h>

int main(int argc, char** argv) {
    if (argc != 2) {
        printf("Usage: ./day1 inputfile.txt\n");
    } else {
        for (int i = 0; i < argc; i++) {
            printf("%s\n", argv[i]);
        }

        FILE *fp = fopen("./day1_input.txt", "r");
        if (!fp) {
            perror("Error opening the file.\n");
            exit(EXIT_FAILURE);
        }
        
        char* tmpStr;
        char* endPtr;
        uintmax_t tmpNum;
        uintmax_t numArr[200];
        int i = 0;
        while (fscanf(fp, "%ms", &tmpStr) > 0) {
            tmpNum = strtoumax(tmpStr, &endPtr, 10);
            free(tmpStr);
            numArr[i] = tmpNum;
            i++;
        }

        for (int i = 0; i < sizeof(numArr) / sizeof(uintmax_t); i++) {
            for (int j = i; j < sizeof(numArr) / sizeof(uintmax_t); j++) {
                for (int k = j; k < sizeof(numArr) / sizeof(uintmax_t); k++) {
                    if (numArr[i] + numArr[j] + numArr[k] == 2020) {
                        printf("num1: %ld, num2: %ld, num3: %ld, produce: %ld\n", 
                                numArr[i], 
                                numArr[j], 
                                numArr[k],
                                numArr[i] * numArr[j] * numArr[k]);
                    }
                }
            }
        }

        fclose(fp);
    }
}
