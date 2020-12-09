// .lines()
use std::io::prelude::*;
use std::collections::HashSet;
use std::collections::HashMap;
use std::collections::VecDeque;

enum InstructionType {
    ACC,
    JMP,
    NOP
}

struct Instruction {
    instruction_type: InstructionType,
    argument: i64,
}

fn parse_instruction(line: String) -> Instruction {
    let mut tokens = line.split(" ");
    let instruction_type = match tokens.next().unwrap() {
        "acc" => InstructionType::ACC,
        "jmp" => InstructionType::JMP,
        _ => InstructionType::NOP,
    };
    let argument = tokens.next().unwrap().parse::<i64>().unwrap();

    return Instruction {
        instruction_type: instruction_type,
        argument: argument,
    }
}

fn parse_input(
    lines : &mut dyn Iterator<Item = Result<String, std::io::Error>>
) -> Vec<Instruction> {
    return lines
        .map(|line| line.unwrap())
        .map(parse_instruction)
        .collect();
}

fn run_program_no_repeat(instructions: Vec<Instruction>) -> i64 {
    let mut visited = HashSet::new();
    let mut current_instruction_position = 0;
    let mut accumulator = 0;

    while !visited.contains(&current_instruction_position) {
        visited.insert(current_instruction_position);
        let current_instruction = &instructions[current_instruction_position];
        match &current_instruction.instruction_type {
            // acc increases or decreases a single global value called the accumulator by
            // the value given in the argument. For example, acc +7 would increase the
            // accumulator by 7. The accumulator starts at 0. After an acc instruction,
            // the instruction immediately below it is executed next.
            InstructionType::ACC => {
                accumulator += current_instruction.argument;
                current_instruction_position += 1;
            },
            // jmp jumps to a new instruction relative to itself. The next instruction to
            // execute is found using the argument as an offset from the jmp instruction;
            // for example, jmp +2 would skip the next instruction, jmp +1 would continue
            // to the instruction immediately below it, and jmp -20 would cause the
            // instruction 20 lines above to be executed next.
            InstructionType::JMP => {
                current_instruction_position = (current_instruction_position as i64
                    + current_instruction.argument) as usize;
            },
            // nop stands for No OPeration - it does nothing. The instruction immediately
            // below it is executed next.
            _ => {
                current_instruction_position += 1;
            },
        }
    }

    return accumulator;
}

fn run_program(instructions: &Vec<Instruction>) -> Option<i64> {
    return Some(0);
}

fn try_program_with_decorrupt(
    index: usize,
    old_instruction: &Instruction,
    new_instruction_type: InstructionType,
    instructions: &mut Vec<Instruction>
) -> Option<i64> {
    let new_instruction = Instruction {
        instruction_type: new_instruction_type,
        argument: old_instruction.argument
    };
    instructions[index] = new_instruction;
    let result = run_program(instructions);
    instructions[index] = *old_instruction;
    return result;
}

fn find_result_without_corruption(instructions: &mut Vec<Instruction>) -> i64 {
    for i in 0..instructions.len() {
        let current_instruction = &instructions[i];
        match &current_instruction.instruction_type {
            InstructionType::JMP => {
                let decorrupt_result = try_program_with_decorrupt(
                    i,
                    current_instruction,
                    InstructionType::NOP,
                    instructions
                );
                match decorrupt_result {
                    Some(result) => return result,
                    None => (),
                }
            },
            InstructionType::NOP => {
                let decorrupt_result = try_program_with_decorrupt(
                    i,
                    current_instruction,
                    InstructionType::JMP,
                    instructions
                );
                match decorrupt_result {
                    Some(result) => return result,
                    None => (),
                }
            },
            _ => (),
        }
    }
    return -1;
}

fn main() {
    let mut instructions = parse_input(&mut std::io::stdin().lock().lines());
    let accumulator = run_program_no_repeat(instructions);
    println!("{}", accumulator);
}
