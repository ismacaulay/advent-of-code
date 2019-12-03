#include <string.h>
#include <stdlib.h>
#include <stdio.h>

int count_memory(char* input) {
    int i, count;
    i = 0;
    count = 0;
    while (1) {
        if (input[i] == ',') {
            count++;
        } else if (input[i] == '\0') {
            count++;
            break;
        }
        i++;
    }

    return count;
}

void init_memory(char* data, int** memory, int* length) {
    int len = count_memory(data);
    *length = len;
    int* mem = (int*)malloc(sizeof(int) * len);

    char* start = data;
    char* runner = data;
    int i = 0;
    while(1) {
        if (*runner == ',') {
            mem[i] = atoi(start);
            start = runner + 1;
            i++;
        } else if (*runner == '\0') {
            mem[i] = atoi(start);
            break;
        }
        runner++;
    }
    *memory = mem;
}

void print_memory(int* program, int length) {
    int i;
    for (i = 0; i < length; i++) {
        printf("%d", program[i]);
        if (i != length-1) {
            printf(",");
        }
    }
}

void run(int* memory) {
    int iptr = 0;
    while(1) {
        int oiptrode = memory[iptr];
        if (oiptrode == 1) {
            int param_addr1 = memory[iptr+1];
            int param_addr2 = memory[iptr+2];
            int output_addr = memory[iptr+3];

            memory[output_addr] = memory[param_addr1] + memory[param_addr2];
        } else if (oiptrode == 2) {
            int param_addr1 = memory[iptr+1];
            int param_addr2 = memory[iptr+2];
            int output_addr = memory[iptr+3];

            memory[output_addr] = memory[param_addr1] * memory[param_addr2];
        } else if (oiptrode == 99) {
            break;
        }

        iptr += 4;
    }
}

void run_test_case(char* input, char* expected) {
    int* memory = NULL;
    int length;

    printf("(%s) should be (%s) => (", input, expected);

    init_memory(input, &memory, &length);
    run(memory);

    print_memory(memory, length);
    printf(")\n");

    free(memory);
}

int main() {
    printf("Test cases:\n\n");
    {
        char input[] = "1,0,0,0,99";
        char expected[] = "2,0,0,0,99";
        run_test_case(input, expected);
    }
    {
        char input[] = "2,3,0,3,99";
        char expected[] = "2,3,0,6,99";
        run_test_case(input, expected);
    }
    {
        char input[] = "2,4,4,5,99,0";
        char expected[] = "2,4,4,5,99,9801";
        run_test_case(input, expected);
    }
    {
        char input[] = "1,1,1,4,99,5,6,0,99";
        char expected[] = "30,1,1,4,2,5,6,0,99";
        run_test_case(input, expected);
    }

    printf("Running on input:\n");
    FILE* file;
    file = fopen("input.txt", "r");

    char input[1024];
    fgets(input, sizeof(input), file);

    int* initial_mem = NULL;
    int length;

    init_memory(input, &initial_mem, &length);
    int* memory = (int*)malloc(sizeof(int) * length);
    memcpy(memory, initial_mem, sizeof(int) * length);

    memory[1] = 12;
    memory[2] = 2;
    run(memory);

    printf("Position 0: %d\n", memory[0]);

    free(memory);

    int noun, verb;
    for (noun = 0; noun <= 99; noun++) {
        for (verb = 0; verb <= 99; verb++) {
            memory = (int*)malloc(sizeof(int) * length);
            memcpy(memory, initial_mem, sizeof(int) * length);

            memory[1] = noun;
            memory[2] = verb;
            run(memory);
            int output = memory[0];
            free(memory);

            if (output == 19690720) {
                printf("Found: noun: %d, verb: %d, (100 * noun + verb) = %d\n", noun, verb, (100*noun) + verb);
                break;
            }
        }
    }

}
