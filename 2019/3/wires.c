#include <limits.h>
#include <stdlib.h>
#include <stdio.h>
#include <string.h>
#include <math.h>

typedef struct Point {
    int x, y, steps;

    struct Point* next;
} Point;

typedef struct PointList {
    struct Point* head;
    struct Point* tail;
} PointList;

Point* find_point(PointList* list, int x, int y) {
   Point* cur = list->head;
   while(cur != NULL) {
       if (cur->x == x && cur->y == y) {
           return cur;
       }

       cur = cur->next;
   }

   return NULL;
}

PointList* build_list(char* path, Point* origin) {
    PointList* list = (PointList*)malloc(sizeof(PointList));

    Point* o = (Point*)malloc(sizeof(Point));
    memcpy(o, origin, sizeof(Point));
    list->head = o;
    list->tail = o;

    int dist_idx = 0;
    int cur_idx = 0;
    int cur_steps = 0;

    char dir;
    while(1) {
        char c = path[cur_idx];
        if (c == ',' || c == '\0') {
            int dist = atoi(path+dist_idx);
            cur_steps += dist;

            int pos_x = list->tail->x;
            int pos_y = list->tail->y;

            if (dir == 'R') {
                pos_x += dist;
            } else if (dir == 'L') {
                pos_x -= dist;
            } else if (dir == 'U') {
                pos_y += dist;
            } else if (dir == 'D') {
                pos_y -= dist;
            }

            Point* p = find_point(list, pos_x, pos_y);
            if (p == NULL) {
                p = (Point*)malloc(sizeof(Point));
                p->x = pos_x;
                p->y = pos_y;
                p->steps = cur_steps;
                p->next = NULL;
            }

            list->tail->next = p;
            list->tail = p;

            if (c == '\0') {
                break;
            }

        } else if (c == 'R' || c == 'L' || c == 'U' || c == 'D') {
            dir = c;
            dist_idx = cur_idx + 1;
        }

        cur_idx++;
    }

    return list;
}

void print_list(PointList* list) {
   Point* cur = list->head;
   while(cur != NULL) {
       printf("(%d, %d, %d)", cur->x, cur->y, cur->steps);
       cur = cur->next;
       if (cur != NULL) {
           printf("->");
       }
   }
   printf("\n");
}

void free_list(PointList* list) {
    Point* cur = list->head;
    while(cur != NULL) {
        Point* doomed = cur;
        cur = cur->next;
        free(doomed);
    }

    list->head = NULL;
    list->tail = NULL;
}

int value_between(int value, int lower, int upper) {
    if (lower > upper) {
        int tmp = lower;
        lower = upper;
        upper = tmp;
    }

    return lower <= value && value <= upper;
}

Point* find_intersection_point(Point* p1, Point* p2, Point* p3, Point* p4) {
    Point* ret = NULL;
    if(value_between(p1->x, p3->x, p4->x) && value_between(p3->y, p1->y, p2->y)) {
        ret = (Point*)malloc(sizeof(Point));
        ret->x = p1->x;
        ret->y = p3->y;
        ret->next = NULL;
    } else if (value_between(p3->x, p1->x, p2->x) && value_between(p1->y, p3->y, p4->y)) {
        ret = (Point*)malloc(sizeof(Point));
        ret->x = p3->x;
        ret->y = p1->y;
        ret->next = NULL;
    }
    return ret;
}

unsigned int calculate_manhatten_dist(Point* p1, Point* p2) {
    unsigned int x_dist = abs(p2->x - p1->x);
    unsigned int y_dist = abs(p2->y - p1->y);

    return x_dist + y_dist;
}

unsigned int find_closest_distance(char* path1, char* path2) {
    Point origin = {0, 0, 0, NULL};

    unsigned int dist = UINT_MAX;

    PointList* wire1 = build_list(path1, &origin);
    PointList* wire2 = build_list(path2, &origin);

    Point* p1 = wire2->head;
    Point* p2 = p1->next;
    while(p2 != NULL) {
        Point* p3 = wire1->head;
        Point* p4 = p3->next;
        while(p4 != NULL) {
            Point* intersect = find_intersection_point(p1, p2, p3, p4);
            if (intersect != NULL) {
                unsigned int d = calculate_manhatten_dist(intersect, &origin);
                if (d > 0 && d < dist) {
                    dist = d;
                }

                free(intersect);
            }

            p3 = p4;
            p4 = p3->next;
        }

        p1 = p2;
        p2 = p1->next;
    }


    free_list(wire1);
    free_list(wire2);

    return dist;
}

int main() {
    {
        char* path1 = "R8,U5,L5,D3";
        char* path2 = "U7,R6,D4,L4";
        unsigned int expected = 6;
        unsigned int dist = find_closest_distance(path1, path2);
        printf("TC: expected=%d, actual=%d\n", expected, dist);
    }
    {
        char* path1 = "R75,D30,R83,U83,L12,D49,R71,U7,L72";
        char* path2 = "U62,R66,U55,R34,D71,R55,D58,R83";
        unsigned int expected = 159;
        unsigned int dist = find_closest_distance(path1, path2);

        printf("TC: expected=%d, actual=%d\n", expected, dist);
    }
    {
        char* path1 = "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51";
        char* path2 = "U98,R91,D20,R16,D67,R40,U7,R15,U6,R7";
        unsigned int expected = 135;
        unsigned int dist = find_closest_distance(path1, path2);

        printf("TC: expected=%d, actual=%d\n", expected, dist);
    }

    FILE* file;
    file = fopen("input.txt", "r");
    if (file == NULL) {
        printf("Input file could not be opened\n");
        exit(EXIT_FAILURE);
    }

    char* path1 = NULL;
    size_t len1 = 0;
    char* path2 = NULL;
    size_t len2 = 0;

    getline(&path1, &len1, file);
    getline(&path2, &len2, file);

    unsigned int dist = find_closest_distance(path1, path2);
    printf("Found dist: %d\n", dist);
}
