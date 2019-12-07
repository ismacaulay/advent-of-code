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

int run(int* memory, int* input) {
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
            int output_addr = memory[iptr+1];
            memory[output_addr] = input[0];
            input++;

            iptr += 2;
        } else if (instruction == 4) {
            // output value
            // 4 <param>
            int mode = parse_mode(opcode, 0);
            int value = get_param_value(mode, memory[iptr+1], memory);
            return value;

            iptr += 2;
        } else if (instruction == 5) {
            // jump-if-true
            // 5 <param> <param>
            int param_1 = get_param_value(parse_mode(opcode, 0), memory[iptr+1], memory);
            int param_2 = get_param_value(parse_mode(opcode, 1), memory[iptr+2], memory);

            if (param_1 != 0) {
                iptr = param_2;
            } else {
                iptr += 3;
            }
        } else if (instruction == 6) {
            // jump-if-false
            // 6 <param> <param>
            int param_1 = get_param_value(parse_mode(opcode, 0), memory[iptr+1], memory);
            int param_2 = get_param_value(parse_mode(opcode, 1), memory[iptr+2], memory);

            if (param_1 == 0) {
                iptr = param_2;
            } else {
                iptr += 3;
            }
        } else if (instruction == 7) {
            // less than
            // 7 <param> <param> <out>
            int param_1 = get_param_value(parse_mode(opcode, 0), memory[iptr+1], memory);
            int param_2 = get_param_value(parse_mode(opcode, 1), memory[iptr+2], memory);
            int output_addr = memory[iptr+3];
            memory[output_addr] = param_1 < param_2 ? 1 : 0;

            iptr += 4;
        } else if (instruction == 8) {
            // equal
            // 8 <param> <param> <out>
            int param_1 = get_param_value(parse_mode(opcode, 0), memory[iptr+1], memory);
            int param_2 = get_param_value(parse_mode(opcode, 1), memory[iptr+2], memory);
            int output_addr = memory[iptr+3];
            memory[output_addr] = param_1 == param_2 ? 1 : 0;

            iptr += 4;
        } else if (instruction == 99) {
            break;
        }
    }
}

int compute_thruster_signal(int* memory, int phase[5]) {
    int signal = 0;
    for (int i = 0; i < 5; i++) {
        int input[2] = {phase[i], signal};
        signal = run(memory, input);
    }
    return signal;
}

int find_max_signal(int* memory, int length, int input_signal, int phase, int* remaining_phases, int num_remaining_phases) {
    int* mem = (int*)malloc(sizeof(int) * length);
    memcpy(mem, memory, sizeof(int) * length);
    int input[] = {phase, input_signal};
    int signal = run(mem, input);
    free(mem);

    if (num_remaining_phases == 0) {
        return signal;
    }

    int max = 0;
    for (int i = 0; i < num_remaining_phases; i++) {
        int phase = remaining_phases[i];

        int* next_phases = (int*)malloc(sizeof(int) * num_remaining_phases - 1);
        int idx = 0;
        for (int j = 0; j < num_remaining_phases; j++) {
            if (j != i) {
                next_phases[idx] = remaining_phases[j];
                idx++;
            }
        }

        int s = find_max_signal(memory, length, signal, phase, next_phases, num_remaining_phases - 1);
        if (s > max) {
            max = s;
        }
        free(next_phases);
    }

    return max;
}

int find_max_thruster_signal(int* initial_memory, int length) {

    int max = 0;
    for(int i = 0; i < 5; i++) {
        int remaining_phases[4];
        int idx = 0;
        for (int j = 0; j < 5; j++) {
            if (j != i) {
                remaining_phases[idx] = j;
                idx++;
            }
        }

        int signal = find_max_signal(initial_memory, length, 0, i, remaining_phases, 4);
        if (signal > max) {
            max = signal;
        }
    }
    return max;
}


void run_test_case(char* input, int expected) {
    int* memory = NULL;
    int length;

    init_memory(input, &memory, &length);

    int signal = find_max_thruster_signal(memory, length);
    printf("(%d) should be (%d)\n", signal, expected);

    free(memory);
}

int main() {
    printf("Test cases:\n");
    {
        char* input = "3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0";
        int expected = 43210;
        run_test_case(input, expected);
    }
    {
        char* input = "3,23,3,24,1002,24,10,24,1002,23,-1,23,101,5,23,23,1,24,23,23,4,23,99,0,0";
        int expected = 54321;
        run_test_case(input, expected);
    }
    {
        char* input = "3,31,3,32,1002,32,10,32,1001,31,-2,31,1007,31,0,33,1002,33,7,33,1,33,31,31,1,32,31,31,4,31,99,0,0,0";
        int expected = 65210;
        run_test_case(input, expected);
    }

    FILE* file;
    file = fopen("input.txt", "r");
    if (file == NULL) {
        printf("Unable to open file\n");
        exit(EXIT_FAILURE);
    }

    char* buf = NULL;
    size_t len = 0;
    getline(&buf, &len, file);

    int* initial_memory = NULL;
    int length;
    init_memory(buf, &initial_memory, &length);
    int signal = find_max_thruster_signal(initial_memory, length);
    printf("Found max thruster signal: %d\n", signal);

    free(initial_memory);
}
