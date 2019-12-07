#include <string.h>
#include <stdlib.h>
#include <stdio.h>

typedef struct Program {
    int* memory;
    int length;
    int iptr;
    int halted;
} Program;

typedef struct Amplifier {
    Program* program;
    int phase;
} Amplifier;

const int NUM_AMPLIFIERS = 5;

void print_memory(int* program, int length) {
    int i;
    for (i = 0; i < length; i++) {
        printf("%d", program[i]);
        if (i != length-1) {
            printf(",");
        }
    }
}

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

Program* create_program(int* memory, int length) {
    Program* program = (Program*)malloc(sizeof(Program));
    program->memory = (int*)malloc(sizeof(int) * length);
    memcpy(program->memory, memory, sizeof(int) * length);
    program->length = length;
    program->iptr = 0;
    program->halted = 0;
    return program;
}

void free_program(Program* program) {
    free(program->memory);
    free(program);
}

Amplifier** create_amplifiers(int* memory, int length) {
    Amplifier** amplifiers = (Amplifier**)malloc(sizeof(Amplifier*) * NUM_AMPLIFIERS);
    for (int i = 0; i < NUM_AMPLIFIERS; i++) {
        Amplifier* amp = (Amplifier*)malloc(sizeof(Amplifier));
        amp->program = create_program(memory, length);
        amp->phase = -1;

        amplifiers[i] = amp;
    }

    return amplifiers;
}

void free_amplifiers(Amplifier** amplifiers) {
    for (int i = 0; i < NUM_AMPLIFIERS; i++) {
        Amplifier* amp = amplifiers[i];
        free_program(amp->program);
        free(amp);
    }

    free(amplifiers);
}

Amplifier** copy_amplifiers(Amplifier** amplifiers) {
    Amplifier** copy = (Amplifier**)malloc(sizeof(Amplifier*) * NUM_AMPLIFIERS);
    for (int i = 0; i < NUM_AMPLIFIERS; i++) {
        Amplifier* amp = (Amplifier*)malloc(sizeof(Amplifier));
        amp->program = create_program(amplifiers[i]->program->memory, amplifiers[i]->program->length);
        amp->phase = amplifiers[i]->phase;
        copy[i] = amp;
    }

    return copy;
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

void run(Program* program, int* input, int* output) {
    if (program->halted) {
        return;
    }

    int* memory = program->memory;
    int iptr = program->iptr;
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
            *output = value;

            iptr += 2;
            break;
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
            program->halted = 1;
            break;
        }
    }

    program->iptr = iptr;
}

int run_pipeline(Amplifier** amplifiers, int input) {
    int output = input;
    for (int i = 0; i < NUM_AMPLIFIERS; i++) {
        Amplifier* amp = amplifiers[i];
        Program* program = create_program(amp->program->memory, amp->program->length);
        int input[] = {amp->phase, output};

        run(program, input, &output);
        free_program(program);
    }
    return output;
}

int run_pipeline_feedback(Amplifier** amplifiers, int input) {
    int output = 0;
    int loops = 0;

    Amplifier** amplifiers_copy = copy_amplifiers(amplifiers);
    while(1) {
        for (int i = 0; i < NUM_AMPLIFIERS; i++) {
            Amplifier* amp = amplifiers_copy[i];
            if (loops == 0) {
                int input[] = {amp->phase, output};
                run(amp->program, input, &output);
            } else {
                int input[] = {output};
                run(amp->program, input, &output);
            }
        }

        if (amplifiers_copy[NUM_AMPLIFIERS-1]->program->halted) {
            break;
        }

        loops++;
    }

    free_amplifiers(amplifiers_copy);
    return output;
}

int find_max_signal(Amplifier** amplifiers, int phase, int* remaining_phases, int num_remaining_phases, int(*runner)(Amplifier**, int)) {
    int idx = 4 - num_remaining_phases;
    amplifiers[idx]->phase = phase;
    if (num_remaining_phases == 0) {
        return runner(amplifiers, 0);
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

        int s = find_max_signal(amplifiers, phase, next_phases, num_remaining_phases - 1, runner);
        if (s > max) {
            max = s;
        }
        free(next_phases);
    }

    return max;
}

int find_max_thruster_signal(Amplifier** amplifiers, int phase_start, int(*runner)(Amplifier**, int)) {
    int max = 0;
    for(int i = phase_start; i < phase_start + NUM_AMPLIFIERS; i++) {
        int remaining_phases[NUM_AMPLIFIERS-1];
        int idx = 0;
        for (int j = 0; j < NUM_AMPLIFIERS; j++) {
            if (j != i-phase_start) {
                remaining_phases[idx] = phase_start + j;
                idx++;
            }
        }
        int signal = find_max_signal(amplifiers, i, remaining_phases, 4, runner);
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
    Amplifier** amplifiers = create_amplifiers(memory, length);

    int signal = find_max_thruster_signal(amplifiers, 0, &run_pipeline);
    printf("(%d) should be (%d)\n", signal, expected);

    free_amplifiers(amplifiers);
    free(memory);
}

void run_feeback_test_case(char* input, int expected) {
    int* memory = NULL;
    int length;

    init_memory(input, &memory, &length);
    Amplifier** amplifiers = create_amplifiers(memory, length);

    int signal = find_max_thruster_signal(amplifiers, 5, &run_pipeline_feedback);
    printf("(%d) should be (%d)\n", signal, expected);

    free_amplifiers(amplifiers);
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
    {
        char* input = "3,26,1001,26,-4,26,3,27,1002,27,2,27,1,27,26,27,4,27,1001,28,-1,28,1005,28,6,99,0,0,5";
        int expected = 139629729;
        run_feeback_test_case(input, expected);
    }
    {
        char* input = "3,52,1001,52,-5,52,3,53,1,52,56,54,1007,54,5,55,1005,55,26,1001,54,-5,54,1105,1,12,1,53,54,53,1008,54,0,55,1001,55,1,55,2,53,55,53,4,53,1001,56,-1,56,1005,56,6,99,0,0,0,0,10";
        int expected = 18216;
        run_feeback_test_case(input, expected);
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
    fclose(file);

    int* memory = NULL;
    int length;
    init_memory(buf, &memory, &length);
    free(buf);

    Amplifier** amplifiers = create_amplifiers(memory, length);
    int signal = find_max_thruster_signal(amplifiers, 0, &run_pipeline);
    printf("Found max thruster signal: %d\n", signal);
    free_amplifiers(amplifiers);

    amplifiers = create_amplifiers(memory, length);
    signal = find_max_thruster_signal(amplifiers, 5, &run_pipeline_feedback);
    printf("Found max thruster signal using feedback pipeline: %d\n", signal);
    free_amplifiers(amplifiers);

    free(memory);
}
