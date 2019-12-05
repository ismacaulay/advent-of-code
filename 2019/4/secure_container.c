#include <stdio.h>

int is_valid(int num) {
    char digits[6];
    sprintf(digits, "%d", num);

    int found_adj = 0;
    for (int i = 1; i < 6; i++) {
        if (digits[i-1] <= digits[i]) {
            if (digits[i-1] == digits[i]) {
                found_adj = 1;
            }
        } else {
            return 0;
        }
    }

    return found_adj;
}

int is_valid_strict(int num) {
    char digits[6];
    sprintf(digits, "%d", num);

    int found_adj = 0;
    int adjacent[] = {0, 0, 0, 0, 0, 0, 0, 0, 0, 0};
    for (int i = 1; i < 6; i++) {
        if (digits[i-1] <= digits[i]) {
            if (digits[i-1] == digits[i]) {
                int idx = digits[i] - '0';
                adjacent[idx]++;
            }
        } else {
            return 0;
        }
    }

    for (int i = 0; i < 10; i++) {
        if (adjacent[i] == 1) {
            return 1;
        }
    }
    return 0;
}

void run(int lower, int upper, int(*check)(int)) {
    int count = 0;
    while (lower <= upper) {
        if (check(lower)) {
            count++;
        }

        lower++;
    }

    printf("Number of valid: %d\n", count);
}

int main() {
    char* input = "168630-718098";

    int lower = 168630;
    int upper = 718098;
    run(lower, upper, &is_valid);
    run(lower, upper, &is_valid_strict);
}
