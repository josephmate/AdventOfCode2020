// .lines()
use std::io::prelude::*;
use std::collections::HashSet;
use std::collections::HashMap;
use std::collections::VecDeque;
use std::env;
use std::cmp::Ordering;

fn solve(
    exit_number: usize,
    starting_numbers: &Vec<usize>
) -> usize {
    let mut prev_number = 0;
    let mut second_most_recently_spoken_times = HashMap::new();
    for i in 0 .. starting_numbers.len() {
        prev_number = starting_numbers[i];
        second_most_recently_spoken_times.insert(prev_number, i);
    }
    let mut most_recently_spoken_times = HashMap::new();

    for i in starting_numbers.len() .. exit_number {
        let next_number = match most_recently_spoken_times.get(&prev_number) {
            // Otherwise, the number had been spoken before; the current player announces how
            // many turns apart the number is from when it was previously spoken.
            Some(most_recently_spoken) => most_recently_spoken - second_most_recently_spoken_times[&prev_number],
            // If that was the first time the number has been spoken, the current player says 0.
            None => 0,
        };
        /*
        println!("=======");
        println!("turn {}", (i+1));
        println!("prev_number {}", prev_number);
        println!("next_number {}", next_number);
        println!("most_recently_spoken {:?}", most_recently_spoken_times.get(&prev_number));
        println!("second_most_spoken {:?}", second_most_recently_spoken_times[&prev_number]);
        println!("second_most_recently_spoken_times {:?}", &second_most_recently_spoken_times);
        println!("most_recently_spoken_times {:?}", &most_recently_spoken_times);
        */
        if !second_most_recently_spoken_times.contains_key(&next_number) {
            second_most_recently_spoken_times.insert(next_number, i);
        } else if !most_recently_spoken_times.contains_key(&next_number) {
            most_recently_spoken_times.insert(next_number, i);
        } else {
            second_most_recently_spoken_times.insert(next_number, most_recently_spoken_times[&next_number]);
            most_recently_spoken_times.insert(next_number, i);
        }
        prev_number = next_number;
    }
    return prev_number;
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let input: Vec<Vec<usize>> = std::io::stdin().lock().lines()
        .map(|line_result| line_result.unwrap())
        .map(|line| line.split(",").map(|v| v.parse::<usize>().unwrap()).collect() )
        .collect();

    
    for starting_numbers in input {
        println!("{}", solve(2020, &starting_numbers));
        println!("{}", solve(30000000, &starting_numbers));
    }

}

