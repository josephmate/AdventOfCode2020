// .lines()
use std::io::prelude::*;
use std::collections::HashSet;
use std::collections::HashMap;
use std::collections::VecDeque;
use std::env;
use std::cmp::Ordering;

#[derive(Debug)]
enum Instruction {
    // and_mask(0s) and or_mask (1s)
    Mask(String),
    // position and value
    Mem(i64, i64),
}

fn parse_mask_str(mask_str: String) -> (i64, i64) {
    let mut and_mask = 0;
    let mut or_mask = 0;

    for c in mask_str.chars() {
        and_mask <<= 1;
        or_mask <<= 1;
        match c {
            '1' => {
                and_mask += 1;
                or_mask += 1;
            },
            '0' => {
                // and_mask is 0
                // or_mask is 0
            },
            _ => {
                and_mask += 1;
                // or_mask is 0
            },
        }
    }

    return (and_mask, or_mask);
}
/*
mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X
mem[8] = 11
mem[7] = 101
mem[8] = 0
*/
fn parse_line(line: String) -> Instruction {
    if line.starts_with("mask") {
        let mut tokens = line.split(" = ");
        tokens.next();
        return Instruction::Mask(tokens.next().unwrap().to_string());
    } else { // assume input is valid
        let mut tokens = line.split(" = ");
        let mut mem_addr_tokens = tokens.next().unwrap().split("[");
        mem_addr_tokens.next();
        let mem_addr = mem_addr_tokens.next().unwrap().split("]").next().unwrap().parse::<i64>().unwrap();
        let mem_value = tokens.next().unwrap().parse::<i64>().unwrap();
        return Instruction::Mem(mem_addr, mem_value);
    }
}

fn solve1(
    instructions: &Vec<Instruction>
) -> i64 {
    // start with a fake mask and assume first line is always the mask
    let mut and_mask = 0;
    let mut or_mask = 0;
    let mut memory = HashMap::new();

    for instruction in instructions {
        match instruction {
            Instruction::Mask(mask_str) => {
                let (new_and_mask, new_or_mask) = parse_mask_str(mask_str.to_string());
                and_mask = new_and_mask;
                or_mask = new_or_mask;
            },
            Instruction::Mem(addr, value) => {
                memory.insert(addr, (value & and_mask) | or_mask);
            },
        }
    }

    return memory.values().sum();
}

fn apply_mask(
    mask: &str,
    addr: i64,
    value: i64,
    memory: &mut HashMap<i64, i64>
) {
    let mut current_bit_mask = 34359738368; // 2^35 then 36th bit is 1;
    let mut mem_to_set: Vec<i64> = Vec::new();
    mem_to_set.push(0);

    for c in mask.chars() {
        let current_bit = addr & current_bit_mask;
        match c {
            '0' => {
                mem_to_set = mem_to_set.iter()
                    .map(|v| v + current_bit)
                    .collect();
                    
            },
            '1' => {
                mem_to_set = mem_to_set.iter()
                    .map(|v| v + current_bit_mask)
                    .collect();
            },
            _ => {
                let mut additional: Vec<i64> = mem_to_set.iter()
                    .map(|v| v + current_bit_mask)
                    .collect();
                mem_to_set.append(&mut additional);
            },
        }

        current_bit_mask >>= 1;
    }

    for mem_posn in mem_to_set {
        memory.insert(mem_posn, value);
    }
}

fn solve2(
    instructions: &Vec<Instruction>
) -> i64 {
    // start with a fake mask and assume first line is always the mask
    let mut mask = "";
    let mut memory = HashMap::new();

    for instruction in instructions {
        match instruction {
            Instruction::Mask(mask_str) => {
                mask = mask_str;
            },
            Instruction::Mem(addr, value) => {
                apply_mask(mask, *addr, *value, &mut memory);
            },
        }
    }

    return memory.values().sum();
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let should_solve1 = args[1].parse::<bool>().unwrap();
    let should_solve2 = args[2].parse::<bool>().unwrap();

    let instructions: Vec<Instruction> = std::io::stdin().lock().lines()
        .map(|line_result| line_result.unwrap())
        .map(parse_line)
        .collect();

    if should_solve1 {
        println!("{}", solve1(&instructions));
    }
    if should_solve2 {
        println!("{}", solve2(&instructions));
    }
}

