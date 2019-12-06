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

int parse_instruction(int opcode) {
    char buf[6];
    int len = sprintf(buf, "%d", opcode);

    int offset = 0;
    if (len > 2) {
        offset = len - 2;
    }

    return atoi(buf + offset);
}

int parse_mode(int opcode, int idx) {
    char buf[6];
    int len = sprintf(buf, "%d", opcode);

    if (idx >= len - 2) {
        return 0;
    }

    return buf[len - 3 - idx] - '0';
}

int get_param_value(int mode, int param, int* memory) {
    if (mode == 0) {
        return memory[param];
    }

    return param;
}

void run(int* memory) {
    int iptr = 0;
    while(1) {
        int opcode = memory[iptr];
        int instruction = parse_instruction(opcode);

        if (instruction == 1) {
            // add
            // 1 <param> <param> <out>
            int param_1 = get_param_value(parse_mode(opcode, 0), memory[iptr+1], memory);
            int param_2 = get_param_value(parse_mode(opcode, 1), memory[iptr+2], memory);
            int output_addr = memory[iptr+3];
            memory[output_addr] = param_1 + param_2;

            iptr += 4;
        } else if (instruction == 2) {
            // multiply
            // 2 <param> <param> <out>
            int param_1 = get_param_value(parse_mode(opcode, 0), memory[iptr+1], memory);
            int param_2 = get_param_value(parse_mode(opcode, 1), memory[iptr+2], memory);
            int output_addr = memory[iptr+3];
            memory[output_addr] = param_1 * param_2;

            iptr += 4;
        } else if (instruction == 3) {
            // read input
            // 3 <out>
            int input;
            printf("Enter input: ");
            scanf("%d", &input);
            printf("\n");
            int output_addr = memory[iptr+1];
            memory[output_addr] = input;

            iptr += 2;
        } else if (instruction == 4) {
            // output value
            // 4 <param>
            int mode = parse_mode(opcode, 0);
            int value = get_param_value(mode, memory[iptr+1], memory);
            printf("%d\n", value);

            iptr += 2;
        } else if (instruction == 99) {
            break;
        }
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
    printf("Test cases:\n");
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
    {
        printf("Instruction: 1 == %d\n", parse_instruction(1));
        printf("Instruction: 2 == %d\n", parse_instruction(2));
        printf("Instruction: 2 == %d\n", parse_instruction(10002));
        printf("Instruction: 99 == %d\n", parse_instruction(10099));
    }
    {
        printf("Mode: 1 == %d\n", parse_mode(100, 0));
        printf("Mode: 1 == %d\n", parse_mode(1000, 1));
        printf("Mode: 1 == %d\n", parse_mode(10000, 2));
        printf("Mode: 0 == %d\n", parse_mode(000, 0));
        printf("Mode: 0 == %d\n", parse_mode(0000, 1));
        printf("Mode: 0 == %d\n", parse_mode(00000, 2));
        printf("Mode: 0 == %d\n", parse_mode(11, 0));
        printf("Mode: 0 == %d\n", parse_mode(11, 1));
        printf("Mode: 0 == %d\n", parse_mode(11, 2));
        printf("Mode: 0 == %d\n", parse_mode(11, 3));
    }

    printf("Running on input:\n");
    FILE* file;
    file = fopen("input.txt", "r");

    char* buf = NULL;
    size_t len = 0;
    getline(&buf, &len, file);

    int* initial_mem = NULL;
    int length;

    init_memory(buf, &initial_mem, &length);
    int* memory = (int*)malloc(sizeof(int) * length);
    memcpy(memory, initial_mem, sizeof(int) * length);

    run(memory);
    free(memory);
}
