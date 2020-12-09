use crate::utils;

#[derive(Debug, Clone)]
struct Instruction {
    op: String,
    arg: String,
    executed: bool,
}

fn read_problem_data() -> Vec<Instruction> {
    let mut result = Vec::new();
    if let Ok(lines) = utils::read_lines("data/day8.txt") {
        for line in lines {
            if let Ok(s) = line {
                let data: Vec<&str> = s.split(" ").collect();
                result.push(Instruction {
                    op: String::from(data[0]),
                    arg: String::from(data[1]),
                    executed: false,
                });
            }
        }
    }

    result
}

#[allow(dead_code)]
pub fn problem1() {
    println!("running problem 8.1:");

    let mut accum: i32 = 0;
    let mut pc: i32 = 0;
    let mut instructions = read_problem_data();

    loop {
        let instruction = &mut instructions[pc as usize];
        if instruction.executed {
            break;
        }

        instruction.executed = true;
        match instruction.op.as_str() {
            "acc" => {
                accum += instruction.arg.parse::<i32>().unwrap();
                pc += 1;
            }
            "jmp" => {
                pc += instruction.arg.parse::<i32>().unwrap();
            }
            "nop" => pc += 1,
            _ => {}
        }
    }

    println!("Accum: {}", accum);
}

#[allow(dead_code)]
pub fn problem2() {
    println!("running problem 8.2:");
    let mut accum: i32 = 0;
    let mut pc: i32 = 0;
    let mut instructions = read_problem_data();

    let mut is_inf = false;
    let mut change_op = false;
    let mut pc_stack = Vec::new();

    loop {
        if (pc as usize) >= instructions.len() {
            break;
        }

        let mut instruction = &mut instructions[pc as usize];
        if instruction.executed {
            // if we found an infinite loop, restore a previous pc, instruction, and accum
            // and mark the program as infinite.
            is_inf = true;
            change_op = true;
            let (old_pc, olc_accum) = pc_stack.pop().unwrap();
            pc = old_pc;
            accum = olc_accum;
            instruction = &mut instructions[pc as usize];
        }

        instruction.executed = true;
        match instruction.op.as_str() {
            "acc" => {
                accum += instruction.arg.parse::<i32>().unwrap();
                pc += 1;
            }
            "jmp" => {
                if !is_inf {
                    pc_stack.push((pc, accum));
                }

                if change_op {
                    change_op = false;
                    pc += 1;
                } else {
                    pc += instruction.arg.parse::<i32>().unwrap();
                }
            }
            "nop" => {
                if !is_inf {
                    pc_stack.push((pc, accum));
                }
                if change_op {
                    change_op = false;
                    pc += instruction.arg.parse::<i32>().unwrap();
                } else {
                    pc += 1;
                }
            }
            _ => {}
        }
    }

    println!("Accum: {}", accum);
}
