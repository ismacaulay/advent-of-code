#include <string.h>
#include <stdlib.h>
#include <stdio.h>
#include <inttypes.h>

typedef struct Block {
    int64_t addr;
    int64_t value;

    struct Block* next;
} Block;

typedef struct Program {
    int64_t* memory;
    int length;
    int iptr;
    int64_t relative_base;
    int halted;

    Block* heap;
} Program;

void print_memory(int64_t* program, int length) {
    int i;
    for (i = 0; i < length; i++) {
        printf("%lld", program[i]);
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

void init_memory(char* data, int64_t** memory, int* length) {
    int len = count_memory(data);
    *length = len;
    int64_t* mem = (int64_t*)malloc(sizeof(int64_t) * len);

    char* start = data;
    char* runner = data;
    int i = 0;
    while(1) {
        char* endptr;
        if (*runner == ',') {
            mem[i] = (int64_t)strtoimax(start, &endptr, 10);
            start = runner + 1;
            i++;
        } else if (*runner == '\0') {
            mem[i] = (int64_t)strtoimax(start, &endptr, 10);
            break;
        }
        runner++;
    }
    *memory = mem;
}

Program* create_program(int64_t* memory, int length) {
    Program* program = (Program*)malloc(sizeof(Program));
    program->memory = (int64_t*)malloc(sizeof(int64_t) * length);
    memcpy(program->memory, memory, sizeof(int64_t) * length);
    program->length = length;
    program->iptr = 0;
    program->halted = 0;
    program->heap = NULL;
    return program;
}

void free_program(Program* program) {
    free(program->memory);

    Block* runner = program->heap;
    while(runner != NULL) {
        Block* doomed = runner;
        runner = runner->next;
        free(doomed);
    }

    free(program);
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
    char buf[12];
    int len = sprintf(buf, "%d", opcode);

    if (idx >= len - 2) {
        return 0;
    }

    return buf[len - 3 - idx] - '0';
}

int64_t get_memory_value(int addr, Program* program) {
    if (addr < 0) {
        return -1;
    }

    if (addr < program->length) {
        return program->memory[addr];
    }

    Block* prev = NULL;
    Block* runner = program->heap;
    while (runner != NULL) {
        if (runner->addr == addr) {
            return runner->value;
        }

        prev = runner;
        runner = runner->next;
    }

    Block* block = (Block*)malloc(sizeof(Block));
    block->addr = addr;
    block->value = 0;
    block->next = NULL;

    if (prev == NULL) {
        program->heap = block;
    } else {
        prev->next = block;
    }
    return 0;
}

void set_memory_value(int addr, int64_t value, Program* program) {
    if (addr < 0) {
        return;
    }

    if (addr < program->length) {
        program->memory[addr] = value;
        return;
    }

    Block* prev = NULL;
    Block* runner = program->heap;
    while (runner != NULL) {
        if (runner->addr == addr) {
            runner->value = value;
            return;
        }

        prev = runner;
        runner = runner->next;
    }


    Block* block = (Block*)malloc(sizeof(Block));
    block->addr = addr;
    block->value = value;
    block->next = NULL;
    if (prev == NULL) {
        program->heap = block;
    } else {
        prev->next = block;
    }
}

int64_t get_param_value(int mode, int64_t addr, Program* program) {
    if (mode == 0) {
        return get_memory_value(addr, program);
    }

    if (mode == 1) {
        return addr;
    }

    if (mode == 2) {
        return get_memory_value(program->relative_base + addr, program);
    }

    printf("Error: unknown mode %d\n", mode);
    return -1;
}

int64_t get_param_addr(int mode, int64_t addr, Program* program) {
    if (mode == 2) {
        return program->relative_base + addr;
    }

    return addr;
}

void run(Program* program, int* input) {
    if (program->halted) {
        return;
    }

    int iptr = program->iptr;
    while(1) {
        int opcode = get_memory_value(iptr, program);
        int instruction = parse_instruction(opcode);

        if (instruction == 1) {
            // add
            // 1 <param> <param> <out>
            int64_t param_addr1 = get_memory_value(iptr+1, program);
            int64_t param_addr2 = get_memory_value(iptr+2, program);
            int64_t param_addr3 = get_memory_value(iptr+3, program);

            int64_t param_1 = get_param_value(parse_mode(opcode, 0), param_addr1, program);
            int64_t param_2 = get_param_value(parse_mode(opcode, 1), param_addr2, program);
            int64_t output = get_param_addr(parse_mode(opcode, 2), param_addr3, program);

            set_memory_value(output, param_1 + param_2, program);

            iptr += 4;
        } else if (instruction == 2) {
            // multiply
            // 2 <param> <param> <out>
            int64_t param_addr1 = get_memory_value(iptr+1, program);
            int64_t param_addr2 = get_memory_value(iptr+2, program);
            int64_t param_addr3 = get_memory_value(iptr+3, program);

            int64_t param_1 = get_param_value(parse_mode(opcode, 0), param_addr1, program);
            int64_t param_2 = get_param_value(parse_mode(opcode, 1), param_addr2, program);
            int64_t output_addr = get_param_addr(parse_mode(opcode, 2), param_addr3, program);
            set_memory_value(output_addr, param_1 * param_2, program);

            iptr += 4;
        } else if (instruction == 3) {
            // read input
            // 3 <out>
            int64_t param = get_memory_value(iptr+1, program);
            int64_t output_addr = get_param_addr(parse_mode(opcode, 0), param, program);
            set_memory_value(output_addr, input[0], program);
            input++;

            iptr += 2;
        } else if (instruction == 4) {
            // output value
            // 4 <param>
            int64_t param_addr = get_memory_value(iptr+1, program);
            int64_t value = get_param_value(parse_mode(opcode, 0), param_addr, program);
            printf("%lld\n", value);

            iptr += 2;
        } else if (instruction == 5) {
            // jump-if-true
            // 5 <param> <param>
            int64_t param_addr1 = get_memory_value(iptr+1, program);
            int64_t param_addr2 = get_memory_value(iptr+2, program);
            int64_t param_1 = get_param_value(parse_mode(opcode, 0), param_addr1, program);
            int64_t param_2 = get_param_value(parse_mode(opcode, 1), param_addr2, program);

            if (param_1 != 0) {
                iptr = param_2;
            } else {
                iptr += 3;
            }
        } else if (instruction == 6) {
            // jump-if-false
            // 6 <param> <param>
            int64_t param_addr1 = get_memory_value(iptr+1, program);
            int64_t param_addr2 = get_memory_value(iptr+2, program);
            int64_t param_1 = get_param_value(parse_mode(opcode, 0), param_addr1, program);
            int64_t param_2 = get_param_value(parse_mode(opcode, 1), param_addr2, program);

            if (param_1 == 0) {
                iptr = param_2;
            } else {
                iptr += 3;
            }
        } else if (instruction == 7) {
            // less than
            // 7 <param> <param> <out>
            int64_t param_addr1 = get_memory_value(iptr+1, program);
            int64_t param_addr2 = get_memory_value(iptr+2, program);
            int64_t output_param = get_memory_value(iptr+3, program);

            int64_t param_1 = get_param_value(parse_mode(opcode, 0), param_addr1, program);
            int64_t param_2 = get_param_value(parse_mode(opcode, 1), param_addr2, program);
            int64_t output_addr = get_param_addr(parse_mode(opcode, 2), output_param, program);

            set_memory_value(output_addr, param_1 < param_2 ? 1 : 0, program);

            iptr += 4;
        } else if (instruction == 8) {
            // equal
            // 8 <param> <param> <out>
            int64_t param_addr1 = get_memory_value(iptr+1, program);
            int64_t param_addr2 = get_memory_value(iptr+2, program);
            int64_t output_param = get_memory_value(iptr+3, program);

            int64_t param_1 = get_param_value(parse_mode(opcode, 0), param_addr1, program);
            int64_t param_2 = get_param_value(parse_mode(opcode, 1), param_addr2, program);
            int64_t output_addr = get_param_addr(parse_mode(opcode, 2), output_param, program);

            set_memory_value(output_addr, param_1 == param_2 ? 1 : 0, program);

            iptr += 4;
        } else if (instruction == 9) {
            // adjust relative base
            // 9 <param>
            int64_t param_addr = get_memory_value(iptr+1, program);
            int mode = parse_mode(opcode, 0);
            int64_t param = get_param_value(parse_mode(opcode, 0), param_addr, program);
            program->relative_base += param;

            iptr += 2;
        } else if (instruction == 99) {
            // halt
            program->halted = 1;
            break;
        }
    }

    program->iptr = iptr;
}

void run_test_case(char* input, char* expected) {
    int64_t* memory = NULL;
    int length;

    printf("TC: Expected output: %s\n", expected);

    init_memory(input, &memory, &length);
    Program* program = create_program(memory, length);
    run(program, NULL);

    free_program(program);
    free(memory);
}

int main() {
    printf("Test cases:\n");
    {
        char* input = "109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99";
        char* expected = "109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99";
        run_test_case(input, expected);
    }
    {
        char* input = "1102,34915192,34915192,7,4,7,99,0";
        char expected[255];
        sprintf(expected, "%lld", (int64_t)34915192 * (int64_t)34915192);
        run_test_case(input, expected);
    }
    {
        char* input = "104,1125899906842624,99";
        char* expected = "1125899906842624";
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
    fclose(file);

    int64_t* memory = NULL;
    int length;
    init_memory(buf, &memory, &length);
    Program* program1 = create_program(memory, length);
    Program* program2 = create_program(memory, length);
    free(buf);

    int input[1] = {1};
    run(program1, input);

    input[0] = 2;
    run(program2, input);

    free_program(program1);
    free_program(program2);
    free(memory);
}
