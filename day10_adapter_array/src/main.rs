// .lines()
use std::io::prelude::*;
use std::collections::HashSet;
use std::collections::HashMap;
use std::collections::VecDeque;
use std::env;

fn solve(
    start_joltage: i64,
    end_joltage: i64,
    start_idx: usize,
    adapters: &Vec<i64>,
    cache: &mut HashMap<(i64, usize), i64>
) -> i64 {

    match cache.get(&(start_joltage, start_idx))
        .map(|entry| entry.clone()) {
            Some(result) => result,
            None => {
                if start_idx == adapters.len()
                        && end_joltage - start_joltage <= 3 {
                    cache.insert((start_joltage, start_idx),1);
                    return 1;
                } else if start_idx == adapters.len()
                        && end_joltage - start_joltage > 3 {
                    
                    cache.insert((start_joltage, start_idx), 0);
                    return 0;
                }

                let mut result = 0;
                for i in start_idx .. adapters.len() {
                    let next_joltage = adapters[i];
                    if next_joltage - start_joltage <= 3 {
                        result += 1 * solve(next_joltage, end_joltage, i+1, adapters, cache);
                    } else {
                        break;
                    }
                }
                cache.insert((start_joltage, start_idx), result.clone());
                return result;
            }
        }
}


fn main() {
    let mut adapters: Vec<i64> = std::io::stdin().lock().lines()
        .map(|line_result| line_result.unwrap())
        .map(|line| line.parse::<i64>().unwrap())
        .collect();
    adapters.sort();
    let built_in_adapter = adapters.iter().max().unwrap();

    let mut current_joltage = 0;
    let mut num_one_diff_joltage = 0;
    let mut num_three_diff_joltage = 1;
    for next_joltage in &adapters {
        if *next_joltage - current_joltage == 1 {
            num_one_diff_joltage += 1;
        } else if *next_joltage - current_joltage == 3 {
            num_three_diff_joltage += 1;
        }
        current_joltage = *next_joltage;
    }

    let result = num_one_diff_joltage * num_three_diff_joltage;
    println!("{}", result);
    println!("{}", solve(
        0,
        *built_in_adapter,
        0,
        &adapters,
        &mut HashMap::new()
    ));
}
