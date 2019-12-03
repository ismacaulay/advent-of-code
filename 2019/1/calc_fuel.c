#include <stdio.h>
#include <math.h>

int calculate_fuel(int mass) {
    unsigned int fuel_amount = (int)floor(mass / 3);
    return fuel_amount - 2;
}

int calculate_total_fuel(int mass) {
    int fuel_amount = calculate_fuel(mass);

    if (fuel_amount <= 0) {
        return 0;
    }
    return fuel_amount + calculate_total_fuel(fuel_amount);
}

int main() {
    printf("Fuel 12 == 2: %d\n", calculate_fuel(14));
    printf("Fuel 14 == 2: %d\n", calculate_fuel(14));
    printf("Fuel 1969 == 654: %d\n", calculate_fuel(1969));
    printf("Fuel 100756 == 33583: %d\n", calculate_fuel(100756));

    printf("Fuel 14 == 2: %d\n", calculate_total_fuel(14));
    printf("Fuel 1969 == 966: %d\n", calculate_total_fuel(1969));
    printf("Fuel 100756 == 50346: %d\n", calculate_total_fuel(100756));

    FILE* file;
    file = fopen("input.txt", "r");

    unsigned int total_fuel = 0;
    unsigned int total_fuel_with_fuel_mass = 0;
    unsigned int mass = 0;
    while (!feof(file)) {
        fscanf(file, "%d\n", &mass);
        total_fuel += calculate_fuel(mass);
        total_fuel_with_fuel_mass += calculate_total_fuel(mass);
    }
    fclose(file);

    printf("Total fuel: %d\n", total_fuel);
    printf("Total fuel with fuel mass: %d\n", total_fuel_with_fuel_mass);
}
