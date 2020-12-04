#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <inttypes.h>
#include <stdbool.h>
#include <string.h>

typedef struct {
    char* pw;
    char character;
    int min;
    int max;
} pw_policy;

pw_policy parse_password_policy(char* policy_string) {

    printf("%s", policy_string);

    char* range = strtok(policy_string, " ");
    char* char_to_check = strtok(NULL, " ");
    char* pw = strtok(NULL, " ");
    char* endPtr;
    uintmax_t min = strtoumax(strtok(range, "-"), &endPtr, 10);
    uintmax_t max = strtoumax(strtok(NULL, "-"), &endPtr, 10);

    /*printf("min: %ld; max: %ld, char: %c\n", min, max, char_to_check[0]);*/

    pw_policy result = {
        .pw = pw, 
        .character = char_to_check[0],
        .min = min,
        .max = max,
    };  
    return result;
}

bool verify_pw(pw_policy policy) {
    int char_count = 0;
    int i = 0;
    char x;
    while ((x = policy.pw[i++])) {
       if (x == policy.character) {  
           char_count++;
       }
    }

    if (char_count >= policy.min && char_count <= policy.max) {
        /*printf("VALID\n");*/
        return true; 
    } else {
        /*printf("INVALID\n");*/
        return false;
    }
}

int main(int argc, char** argv) {
    if (argc != 2) {
        printf("Usage: ./day2 inputfile.txt\n");
    } else {
        for (int i = 0; i < argc; i++) {
            printf("%s\n", argv[i]);
        }

        FILE *fp = fopen(argv[1], "r");
        if (!fp) {
            perror("Error opening the file.\n");
            exit(EXIT_FAILURE);
        }

        // Parse file
        char* line;
        pw_policy tmpPolicy;
        size_t len = 0;
        int valid_count = 0;
        while (getline(&line, &len, fp) > 0) {
            tmpPolicy = parse_password_policy(line);
             if (verify_pw(tmpPolicy)) {
                 valid_count++;
             }
        }

        printf("VALID passwords: %d\n", valid_count);

        fclose(fp);
    }
}
