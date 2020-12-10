// .lines()
use std::io::prelude::*;
use std::collections::HashSet;
use std::collections::HashMap;
use std::collections::VecDeque;
use std::env;

fn find_all_pairs(subset : &[i64]) -> HashSet<i64> {
    let mut result = HashSet::new();
    for i in 0 .. subset.len() {
        for j in 0 .. subset.len() {
            if i != j {
                result.insert(subset[i] + subset[j]);
            }
        }
    }
    return result;
}

fn find_invalid_number(
    preamble_legnth: usize,
    numbers: &Vec<i64>
) -> i64 {

    let mut checksum = find_all_pairs(&numbers[0..preamble_legnth]);

    for i in preamble_legnth .. numbers.len() {
        if !checksum.contains(&numbers[i]) {
            return numbers[i];
        }
        checksum = find_all_pairs(
            &numbers[ (i-preamble_legnth)..(i+1) ]
        );
    }

    return -1;
}

fn find_contiguous_sum(
    invalid_number: i64,
    numbers: &Vec<i64>
) -> i64 {

    for i in 0 .. numbers.len() - 1 {
        for j in (i+2) ..= numbers.len() {
            let contiguous_subarray = &numbers[ i..j ];
            if contiguous_subarray.iter().sum::<i64>() == invalid_number {
                return contiguous_subarray.iter().min().unwrap()
                    + contiguous_subarray.iter().max().unwrap();
            }
        }
    }

    return 0;
}

fn main() {
    let mut args = env::args();
    args.next(); // 0th argument is the program
    let preamble_legnth = args.next().unwrap().parse::<usize>().unwrap();
    let numbers = std::io::stdin().lock().lines()
        .map(|line_result| line_result.unwrap())
        .map(|line| line.parse::<i64>().unwrap())
        .collect();
    let invalid_number = find_invalid_number(preamble_legnth, &numbers);
    println!("{}", invalid_number);
    let contiguous_sum_min_max = find_contiguous_sum(invalid_number, &numbers);
    println!("{}", contiguous_sum_min_max);
}
