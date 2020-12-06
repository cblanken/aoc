#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <inttypes.h>
#include <stdbool.h>
#include <string.h>

typedef struct passport {
    char* byr;
    char* iyr;
    char* eyr;
    char* hgt;
    char* hcl;
    char* ecl;
    char* pid;
    char* cid;
} Passport;

void free_pp(Passport* pp) {
    if (!strcmp(pp->byr, "") && pp->byr != NULL) free(pp->byr);
    if (!strcmp(pp->iyr, "") && pp->iyr != NULL) free(pp->byr);
    if (!strcmp(pp->eyr, "") && pp->eyr != NULL) free(pp->byr);
    if (!strcmp(pp->hgt, "") && pp->hgt != NULL) free(pp->byr);
    if (!strcmp(pp->hcl, "") && pp->hcl != NULL) free(pp->byr);
    if (!strcmp(pp->ecl, "") && pp->ecl != NULL) free(pp->byr);
    if (!strcmp(pp->pid, "") && pp->pid != NULL) free(pp->byr);
    if (!strcmp(pp->cid, "") && pp->cid != NULL) free(pp->byr);
}

Passport* init_pp() {
    Passport* pp = (Passport*)malloc(sizeof(Passport));
    pp->byr = "";
    pp->iyr = "";
    pp->eyr = "";
    pp->hgt = "";
    pp->hcl = "";
    pp->ecl = "";
    pp->pid = "";
    pp->cid = "";
    return pp;
}

void print_pp(Passport* pp) {
    printf("pp: %s, %s, %s, %s, %s, %s, %s, %s\n", 
            pp->byr, pp->iyr, pp->eyr, pp->hgt, pp->hcl, pp->ecl, pp->pid, pp->cid);
}

Passport* update_passport_field(Passport* pp, char* field) {
    char* field_name = strdup(field);
    char* saveptr;
    char* val = strtok_r(field_name, ":", &saveptr);
    val = strtok_r(NULL, ":", &saveptr);

    /*printf("field_name: %s\n", field_name);*/
    /*printf("val: %s\n", val);*/
    if (strncmp(field_name, "byr", 3) == 0) {
        pp->byr = val;
    }
    else if (strncmp(field_name, "iyr", 3) == 0) {
        pp->iyr = val;
    }
    else if (strncmp(field_name, "eyr", 3) == 0) {
        pp->eyr = val;
    }
    else if (strncmp(field_name, "hgt", 3) == 0) {
        pp->hgt = val;
    }
    else if (strncmp(field_name, "hcl", 3) == 0) {
        pp->hcl = val;
    }
    else if (strncmp(field_name, "ecl", 3) == 0) {
        pp->ecl = val;
    }
    else if (strncmp(field_name, "pid", 3) == 0) {
        pp->pid = val;
    }
    else if (strncmp(field_name, "cid", 3) == 0) {
        pp->cid = val;
    }

    return pp;
}

bool pp_is_valid (Passport* pp) {
    // Ignore cid fields
    return !(strcmp(pp->byr, "") == 0 ||
            strcmp(pp->iyr, "")  == 0 ||
            strcmp(pp->eyr, "")  == 0 ||
            strcmp(pp->hgt, "")  == 0 ||
            strcmp(pp->hcl, "")  == 0 ||
            strcmp(pp->ecl, "")  == 0 ||
            strcmp(pp->pid, "")  == 0);
}

Passport* parse_passport_line(Passport* pp, char* line) {
    char* field = strdup(line);

    // Remove trailing newline from field
    size_t line_size = strcspn(field, "\n");
    field[line_size] = '\0';

    // Initial tokenization of field line
    char* saveptr;
    strtok_r(field, " ", &saveptr);

    do {
        pp = update_passport_field(pp, field);
    } while((field = strtok_r(NULL, " ", &saveptr)));
    
    return pp;
}

int main(int argc, char** argv) {
    if (argc != 2) {
        printf("Usage: ./day4 inputfile.txt\n");
    } else {
        for (int i = 0; i < argc; i++) {
            printf("%s\n", argv[i]);
        }

        FILE *fp = fopen(argv[1], "r");
        if (!fp) {
            perror("Error opening the file.\n");
            exit(EXIT_FAILURE);
        }

        // Count passport records
        char* line = "";
        size_t len = 0;
        uint32_t passport_count = 0; 
        while (getline(&line , &len, fp) > 0) {
            if (strcmp(line, "\n") == 0) {
                passport_count++;
            }
        }
        rewind(fp);
        
        // Parse input file
        Passport* pp_buff = init_pp();
        uint32_t valid_pp_count = 0;

        for (int i = 0; i < passport_count + 1; i++) {
            while (getline(&line, &len, fp) > 0) {
                if (!strncmp(line, "\n", 1)) break;
                pp_buff = parse_passport_line(pp_buff, line);             
            }

            /*print_pp(pp_buff);*/
            if (pp_is_valid(pp_buff)) {
                valid_pp_count++;
                /*printf("VALID\n");*/
            }
            else {
                /*printf("INVALID\n");*/
            }

            pp_buff = init_pp(); 
        }
        
        printf("%u VALID Passports out of %u.\n", valid_pp_count, passport_count);
        free(line);
       
        fclose(fp);
    }
}
