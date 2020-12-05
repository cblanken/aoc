#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <inttypes.h>
#include <stdbool.h>
#include <string.h>

typedef struct point {
    uint32_t x;
    uint32_t y;
} Point;

int main(int argc, char** argv) {
    if (argc != 2) {
        printf("Usage: ./day3 inputfile.txt\n");
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
        char* line = "";
        size_t len = 0;
        uint32_t map_width = 0;
        if (getline(&line, &len, fp) > 0) {
            map_width = strlen(line);
            rewind(fp);
        } else {
            printf("ERROR: can't read map data file!\n");
        }

        // Parse input file 
        char* map[323];
        uint32_t i = 0;
        while (getline(&line, &len, fp) > 0) {
            map[i] = strdup(line);
            i++;
        }
        free(line);
        uint32_t line_count = i;
        
        // Initialize slope data
        Point slopes[] = {
            [0] = {.x = 1, .y = 1},
            [1] = {.x = 3, .y = 1},
            [2] = {.x = 5, .y = 1},
            [3] = {.x = 7, .y = 1},
            [4] = {.x = 1, .y = 2}
        };

        for (int s = 0; s < sizeof(slopes) / sizeof(Point); s++) {
            // Generate complete map
            const Point slope = {.x = slopes[s].x, .y = slopes[s].y};
            char* output_map[323];
            uint32_t map_repeat_count = (323 / slope.y * slope.x / (map_width - 1)) + 1;
            for (int i = 0; i < 323; i++) {
                // map_width - 1 to ignore '\n'
                char* buf = strndup(map[i], map_width - 1);
                uint32_t fullLineSize = strlen(buf) * map_repeat_count;
                char* fullLine = (char*)malloc(fullLineSize);
                for (int j = 0; j < map_repeat_count; j++) {
                    fullLine = strncat(fullLine, buf, map_width);
                }
                /*printf("%s\n", fullLine);*/
                output_map[i] = fullLine;
            }

            // Walk map with slope
            uint32_t spaces = 0;
            uint32_t trees = 0;
            Point p = {.x = 0, .y = 0};
            uint32_t y_progress = 0;
            uint32_t x_progress = 0;
            while(y_progress < line_count - 1) {
                y_progress += slope.y;
                x_progress += slope.x;
                p.x = (x_progress) % (map_width - 1);
                p.y = (y_progress) % (map_width - 1);
                char* current_line = strdup(map[y_progress]);

                // Count 'trees' and 'spaces' and update output_map
                if (map[y_progress][p.x] == '#') {
                    trees++;
                    current_line[p.x] = 'X';
                    output_map[y_progress][x_progress] = 'X';
                    /*printf("tree!(%u): \t", trees);*/
                }
                else if(map[p.y][p.x] == '.') {
                    spaces++;
                    current_line[p.x] = 'O';
                    output_map[y_progress][x_progress] = 'O';
                    /*printf("space(%u): \t", spaces);*/
                }
                else {
                    /*printf("ERROR: \t");*/
                }

                free(current_line);
            }


            // Output Results
            for (int i = 0; i < y_progress + 1; i++) {
                /*printf("%s\n", output_map[i]);*/
            }
            printf("trees: %u; spaces: %u\n", trees, spaces);
            
        }


        // Cleanup
        for (int i = 0; i < line_count; i++) {
            free(map[i]);
        }

        fclose(fp);
    }
}
