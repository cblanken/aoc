#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <inttypes.h>
#include <stdbool.h>
#include <string.h>
#include <regex.h>

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
    regex_t hcl_re, ecl_re, pid_re;

    char* endPtr;
    // Years
    uintmax_t byr = strtoumax(pp->byr, &endPtr, 10);
    uintmax_t iyr = strtoumax(pp->iyr, &endPtr, 10);
    uintmax_t eyr = strtoumax(pp->eyr, &endPtr, 10);

    // Height
    uintmax_t hgt = strtoumax(pp->hgt, &endPtr, 10);
    char hgt_unit[3]; 
    snprintf(hgt_unit, 3, "%s", pp->hgt + strlen(pp->hgt) - 2);

    // Hair Color (Hex/RGB)
    char* hcl = pp->hcl;
    char* hcl_pattern = "#([0-9a-f]{6})";
    regcomp(&hcl_re, hcl_pattern, REG_EXTENDED);

    // Eye Color (set)
    char* ecl = pp->ecl;
    char* ecl_pattern = "(amb|blu|brn|gry|grn|hzl|oth)";
    regcomp(&ecl_re, ecl_pattern, REG_EXTENDED);

    // Passport ID
    char* pid = pp->pid;
    char* pid_pattern = "^[0-9]{9}$";
    regcomp(&pid_re, pid_pattern, REG_EXTENDED);

    printf("byr: %lu, iyr: %lu, eyr: %lu, hgt: %lu, hgt_unit: %s, hcl: %s, hcl_patt: %s, "
           "ecl: %s, ecl_patt: %s, pid: %s, pid_patt: %s\n",
            byr, iyr, eyr, hgt, hgt_unit, hcl, hcl_pattern, ecl, ecl_pattern, pid, pid_pattern);

    // Ignore cid fields
    return ((byr > 999 && byr <= 9999 && byr >= 1920 && byr <= 2002) &&
            (iyr > 999 && iyr <= 9999 && iyr >= 2010 && iyr <= 2020) &&
            (eyr > 999 && eyr <= 9999 && eyr >= 2020 && eyr <= 2030) &&
            ((!strcmp(hgt_unit, "in") && hgt >= 59 && hgt <= 76) || (!strcmp(hgt_unit, "cm") && hgt >= 150 && hgt <= 193)) &&
            regexec(&hcl_re, hcl, 0, NULL, 0) == 0 &&
            regexec(&ecl_re, ecl, 0, NULL, 0) == 0 && 
            regexec(&pid_re, pid, 0, NULL, 0) == 0
           );
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
