#include <stdio.h>
#include <string.h>
#include <stdlib.h>

typedef struct Asteroid {
    int x, y;
} Asteroid;

typedef struct AsteroidMap {
    char map[128][128];
    int width, height;
} AsteroidMap;

int find_divisor(int a, int b) {
    int start = a < b ? a : b;
    for (int i = start; i > 0; i--) {
        if (a % i == 0 && b % i == 0) {
            return i;
        }
    }

    return 1;
}

int hits(AsteroidMap* map, Asteroid a, Asteroid b) {
    int start_x = a.x;
    int start_y = a.y;
    int end_x = b.x;
    int end_y = b.y;

    /* printf("\n\t[hits]%d %d %d %d\n", start_x, start_y, end_x, end_y); */

    if (start_x == end_x) {
        int step = start_y < end_y ? 1 : -1;
        start_y += step;
        while (abs(start_y) != abs(end_y)) {
            if (map->map[start_y][start_x] == '#') {
                return 0;
            }

            start_y += step;
        }
    } else if (start_y == end_y) {
        int step = start_x < end_x ? 1 : -1;
        start_x += step;
        while (abs(start_x) != abs(end_x)) {
            if (map->map[start_y][start_x] == '#') {
                return 0;
            }

            start_x += step;
        }
    } else {
        int x_diff = abs(a.x - b.x);
        int y_diff = abs(a.y - b.y);


        int divisor = find_divisor(x_diff, y_diff);

        int x_step = x_diff / divisor;
        int y_step = y_diff / divisor;

        x_step *= a.x < b.x ? 1 : -1;
        y_step *= a.y < b.y ? 1 : -1;
        int hit_another = 0;
        start_x += x_step;
        start_y += y_step;

        /* printf("\t[hits] %d %d\n", x_step, y_step); */
        while(1) {
            /* printf("\t[hits] %d %d\n", start_x, start_y); */
            if (start_x == end_x && start_y == end_y) {
                return !hit_another;
            }

            if (x_step < 0 && start_x < end_x) {
                return 0;
            } else if (x_step > 0 && start_x > end_x) {
                return 0;
            }

            if (y_step < 0 && start_y < end_y) {
                return 0;
            } else if (y_step > 0 && start_y > end_y) {
                return 0;
            }

            if (map->map[start_y][start_x] == '#') {
                hit_another = 1;
            }

            start_x += x_step;
            start_y += y_step;
        }
    }
    return 1;
}

int calc_num_seen(AsteroidMap* map, Asteroid current) {
    int count = 0;

    for (int y = 0; y < map->height; y++) {
        for (int x = 0; x < map->width; x++) {
            // dont check yourself
            if (y == current.y && x == current.x) {
                continue;
            }

            // dont check empty spaces
            if (map->map[y][x] == '.') {
                continue;
            }

            Asteroid check = {x, y};
            if (hits(map, current, check)) {
                count++;
            }
        }
    }

    return count;
}

int calc_most_seen(AsteroidMap* map) {
    int count = 0;
    int max_x = 0;
    int max_y = 0;
    for (int y = 0; y < map->height; y++) {
        for (int x = 0; x < map->width; x++) {
            // dont check empty spaces
            if (map->map[y][x] == '.') {
                continue;
            }

            Asteroid current = {x, y};
            int can_see = calc_num_seen(map, current);
            if (can_see > count) {
                count = can_see;
                max_x = x;
                max_y = y;
            }
        }
    }

    return count;
}

AsteroidMap* read_asteroid_map(char* fname) {
    FILE* file;
    file = fopen(fname, "r");
    if (file == NULL) {
        printf("Error: could not open file %s\n", fname);
        exit(EXIT_FAILURE);
    }

    AsteroidMap* map = (AsteroidMap*)malloc(sizeof(AsteroidMap));

    char line[128];
    int rows = 0;
    int cols = 0;
    while(fgets(line, sizeof(line), file) != NULL) {
        if (cols == 0) {
            cols = strlen(line) - 1;
        }
        line[cols] = '\0';

        memcpy(map->map[rows], line, sizeof(line));
        rows++;
    }
    fclose(file);

    map->width = cols;
    map->height = rows;
    return map;
}

void run_test(char* fname, char* expected) {
    AsteroidMap* map = read_asteroid_map(fname);
    int most = calc_most_seen(map);
    printf("TC: %s == %d\n", expected, most);

    free(map);
}

int main() {
    run_test("test1.txt", "(3, 4) => 8");
    run_test("test2.txt", "(5, 8) => 33");
    run_test("test3.txt", "(1, 2) => 35");
    run_test("test4.txt", "(6, 3) => 41");
    run_test("test5.txt", "(11, 13) => 210");

    run_test("input.txt", "");
}
