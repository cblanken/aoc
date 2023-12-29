#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <inttypes.h>
#include <stdbool.h>
#include <string.h>
#include <regex.h>


char* decode_seat_id(char* seat_id) {
    char* buf = strdup(seat_id);
    for (int i = 0; i < strlen(seat_id); i++) {
        if (buf[i] == 'B' || buf[i] == 'R') {
            buf[i] = '1';
        } else if (buf[i] == 'F' || buf[i] == 'L') {
            buf[i] = '0';
        }
    }

    return buf;
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

        // Parse input file
        const int seat_columns = 8;
        const int seat_rows = 128;
        int seats_taken[128][8] = {0};

        // First 7 bits of input (F or B)
        uintmax_t row = 0;
        char row_buff[8];

        // Final 3 bits of input (L or R)
        uintmax_t col = 0;
        char col_buff[4];

        char* endptr;
        char* line = "";
        size_t len = 0;
        uintmax_t max_seat_id = 0;
        uintmax_t min_seat_id = 127 * 8 + 8;
        while (getline(&line , &len, fp) > 0) {
            snprintf(row_buff, 8, "%s", line);    
            snprintf(col_buff, 4, "%s", line + 7);
            
            row = strtoumax(decode_seat_id(row_buff), &endptr, 2);
            col = strtoumax(decode_seat_id(col_buff), &endptr, 2);

            uintmax_t new_seat_id = row * 8 + col;
            if (new_seat_id > max_seat_id) {
                max_seat_id = new_seat_id;
            }
            
            if (new_seat_id < min_seat_id) {
                min_seat_id = new_seat_id;
            }

            // Track every seat taken 
            seats_taken[row][col] = 1;

            printf("row: %lu, col: %lu, id: %lu\n", row, col, row * 8 + col);

        }
        printf("MAX SEAT ID: %lu\n", max_seat_id);
        printf("MIN SEAT ID: %lu\n", min_seat_id);
        
        free(line);
       
        fclose(fp);

        // Find my seat
        for (int i = 0; i < seat_rows; i++)
        {
            for (int j = 0; j < seat_columns; j++)
            {
               if (seats_taken[i][j] != 1)
               {
                   printf("Seat %d row: %d, column: %d not taken.\n", i * 8 + j, i, j);
               }
            }
            
        }
    }
}
