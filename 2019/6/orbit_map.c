#include <stdio.h>
#include <stdlib.h>
#include <string.h>

typedef struct Object {
    char* name;

    struct Object* orbitting;
} Object;

typedef struct ObjectNode {
    Object* object;

    struct ObjectNode* next;
} ObjectNode;

typedef struct OrbitMap {
    ObjectNode* head;
    ObjectNode* tail;
} OrbitMap;

void print_list(ObjectNode* head) {
    ObjectNode* runner = head;
    while(runner != NULL) {
        Object* obj = runner->object;

        printf("%s ", obj->name);
        if (obj->orbitting != NULL) {
            printf(" -> %s", obj->orbitting->name);
        }

        printf("\n");

        runner = runner->next;
    }
}

void print_orbit(Object* obj) {
    Object* runner = obj;
    while(runner != NULL) {
        printf("%s", runner->name);
        if (runner->orbitting != NULL) {
            printf(" -> ");
        }

        runner = runner->orbitting;
    }
    printf("\n");
}

ObjectNode* insert_object(OrbitMap* map, char* name) {
    Object* obj = (Object*)malloc(sizeof(Object));
    obj->name = (char*)malloc(sizeof(name));
    memcpy(obj->name, name, sizeof(name));
    obj->orbitting = NULL;

    ObjectNode* node = (ObjectNode*)malloc(sizeof(ObjectNode));
    node->object = obj;
    node->next = NULL;

    if (map->tail == NULL) {
        map->head = node;
        map->tail = node;
    } else {
        map->tail->next = node;
        map->tail = node;
    }

    return node;
}

ObjectNode* find_object(OrbitMap* map, char* name) {
    ObjectNode* runner = map->head;
    while(runner != NULL) {
        if (strcmp(runner->object->name, name) == 0) {
            break;
        }

        runner = runner->next;
    }

    return runner;
}

OrbitMap* build_orbit_map(char* fname) {
    FILE* file;
    file = fopen(fname, "r");
    if (file == NULL) {
        printf("Count not find file: %s\n", fname);
        exit(EXIT_FAILURE);
    }

    OrbitMap* map = (OrbitMap*)malloc(sizeof(OrbitMap));
    map->head = NULL;
    map->tail = NULL;

    char object[8];
    char orbiter[8];
    while(fscanf(file, "%[^)])%s\n", &object, &orbiter) != EOF) {
        ObjectNode* orbiter_node = find_object(map, orbiter);
        if (orbiter_node == NULL) {
            orbiter_node = insert_object(map, orbiter);
        }

        ObjectNode* object_node = find_object(map, object);
        if (object_node == NULL) {
            object_node = insert_object(map, object);
        }

        orbiter_node->object->orbitting = object_node->object;
    }

    fclose(file);
    return map;
}

void free_orbit_map(OrbitMap* map) {
    ObjectNode* runner = map->head;
    while (runner != NULL) {
        ObjectNode* doomed = runner;
        runner = runner->next;

        free(doomed->object);
        free(doomed);
    }

    map->head = NULL;
    map->tail = NULL;
}

int get_checksum(OrbitMap* map) {
    int orbits = 0;
    int indirect_orbits = 0;
    ObjectNode* runner = map->head;
    while(runner != NULL) {
        if (strcmp(runner->object->name, "COM") != 0) {
            orbits++;
        }

        Object* obj_runner = runner->object->orbitting;
        while(obj_runner != NULL) {
            if (obj_runner->orbitting != NULL) {
                indirect_orbits++;
            }
            obj_runner = obj_runner->orbitting;
        }

        runner = runner->next;
    }

    return orbits + indirect_orbits;
}


int main() {
    {
        char* input = "test.txt";
        OrbitMap* map = build_orbit_map(input);
        int orbits = get_checksum(map);
        printf("Checksum for map in %s: %d\n", input, orbits);
        free_orbit_map(map);
    }
    {
        char* input = "test2.txt";
        OrbitMap* map = build_orbit_map(input);
        int orbits = get_checksum(map);
        printf("Checksum for map in %s: %d\n", input, orbits);
        free_orbit_map(map);
    }
    {
        char* input = "input.txt";
        OrbitMap* map = build_orbit_map(input);
        int orbits = get_checksum(map);
        printf("Checksum for map in %s: %d\n", input, orbits);
        free_orbit_map(map);
    }
}
