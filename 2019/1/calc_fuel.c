#include <stdio.h>
#include <math.h>

unsigned int calculate_fuel(unsigned int mass) {
    unsigned int fuel_amount = (unsigned int)floor(mass / 3);
    return fuel_amount - 2;
}

int main() {
    FILE* file;
    file = fopen("input.txt", "r");

    unsigned int total_fuel = 0;
    unsigned int mass = 0;
    while (!feof(file)) {
        fscanf(file, "%d\n", &mass);
        total_fuel += calculate_fuel(mass);
    }
    fclose(file);

    printf("Total fuel: %d\n", total_fuel);
}
