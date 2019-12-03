#include <stdio.h>
#include <math.h>

int calculate_fuel(int mass) {
    int fuel_amount = (int)floor(mass / 3);
    fuel_amount -= 2;

    if (fuel_amount <= 0) {
        return 0;
    }
    return fuel_amount + calculate_fuel(fuel_amount);
}

int main() {
    /* tests
    printf("Fuel 14: %d\n", calculate_fuel(14));
    printf("Fuel 1969: %d\n", calculate_fuel(1969));
    printf("Fuel 100756: %d\n", calculate_fuel(100756));
    */

    FILE* file;
    file = fopen("input.txt", "r");

    int total_fuel = 0;
    int mass = 0;
    while (!feof(file)) {
        fscanf(file, "%d\n", &mass);
        total_fuel += calculate_fuel(mass);
    }
    fclose(file);

    printf("Total fuel: %d\n", total_fuel);
}
