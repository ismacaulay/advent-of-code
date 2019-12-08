#include <stdio.h>
#include <stdlib.h>
#include <limits.h>
#include <string.h>

const int NUM_COLS = 25;
const int NUM_ROWS = 6;

int count_digits(char* layer, char digit) {
    int count = 0;
    for (int i = 0; i < NUM_ROWS * NUM_COLS; i++) {
        if (layer[i] == digit) {
            count++;
        }
    }
    return count;
}

char* decode_image(char** layers, int num_layers) {
    char* image = (char*)malloc(sizeof(char) * NUM_COLS * NUM_ROWS);

    for (int i = 0; i < NUM_ROWS * NUM_COLS; i++) {
        for (int j = 0; j < num_layers; j++) {
            char color = layers[j][i];
            if (color != '2') {
                image[i] = color;
                break;
            }
        }
    }

    return image;
}

void write_image(char* image) {
    for (int i = 0; i < NUM_ROWS; i++) {
        for (int j = 0; j < NUM_COLS; j++) {
            printf("%c", image[(NUM_COLS * i) + j]);
        }
        printf("\n");
    }
}

int main() {
    FILE* file;
    file = fopen("input.txt", "r");
    if (file == NULL) {
        printf("Unable to open file\n");
        exit(EXIT_FAILURE);
    }

    char* buf = NULL;
    size_t len = 0;
    size_t count = getline(&buf, &len, file);
    fclose(file);

    int num_layers = count / (NUM_COLS * NUM_ROWS);
    int layer_size = NUM_ROWS * NUM_COLS;
    char* layers[num_layers];
    int layer_with_least_zeros = 0;
    int min_zeros = INT_MAX;
    for (int i = 0; i < num_layers; i++) {
        layers[i] = (char*)malloc(sizeof(char) * layer_size);
        memcpy(layers[i], buf + (i * layer_size), sizeof(char) * layer_size);

        int num_zeros = count_digits(layers[i], '0');
        if (num_zeros < min_zeros) {
            min_zeros = num_zeros;
            layer_with_least_zeros = i;
        }
    }
    free(buf);

    int num_ones = count_digits(layers[layer_with_least_zeros], '1');
    int num_twos = count_digits(layers[layer_with_least_zeros], '2');
    printf("num ones * num twos = %d\n", num_ones * num_twos);

    char* image = decode_image(layers, num_layers);
    write_image(image);
    free(image);

    for (int i = 0; i < num_layers; i++) {
        free(layers[i]);
    }
}

