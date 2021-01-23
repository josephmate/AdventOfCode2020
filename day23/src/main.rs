#![allow(unused_imports)]
// .lines()
use std::io::prelude::*;
use std::collections::HashSet;
use std::collections::HashMap;
use std::collections::VecDeque;
use std::env;
use std::cmp::Ordering;

fn parse_input(
  lines: &mut dyn Iterator<Item=String>
) -> Vec<usize> {
  lines.next().unwrap().chars()
    .map(|c| c.to_digit(10).unwrap() as usize)
    .collect()
}

/*
The crab selects a destination cup: the cup with a label equal to the
current cup's label minus one. If this would select one of the cups
that was just picked up, the crab will keep subtracting one until it
finds a cup that wasn't just picked up. If at any point in this process
the value goes below the lowest value on any cup's label, it wraps
around to the highest value on any cup's label instead.
*/
fn calc_destination_optimized(
  current_cup: usize,
  picked_up_cups: Vec<usize>,
  max_value: usize
) -> usize {
  let picked_up_cups = picked_up_cups.iter().copied().collect::<HashSet<usize>>();

  let mut numbers_to_try = Vec::new();
  let mut current_value = current_cup;
  for _ in 1..=4 {
    current_value -= 1;
    if current_value < 1 {
      current_value = max_value;
    }
    numbers_to_try.push(current_value);
  }

  for i in numbers_to_try {
    if !picked_up_cups.contains(&i) {
      return i;
    }
  }
  // should never return this since
  // the size of picked_up_cups is 3
  // and the size of numbers_to_try is 4
  0
}

fn solve(
  input: &[usize],
  extras: usize,
  iterations: usize,
  max_size: usize
) -> usize {
  let mut cups = input.iter().copied().collect::<VecDeque<usize>>();

  for i in 10..=extras {
    cups.push_back(i);
  }

  for _ in 0..iterations {
    /*
    The crab picks up the three cups that are immediately clockwise of the
    current cup. They are removed from the circle; cup spacing is adjusted
    as necessary to maintain the circle.
    */
    let current_cup = cups.pop_front().unwrap();
    cups.push_back(current_cup);
    let first = cups.pop_front().unwrap();
    let second = cups.pop_front().unwrap();
    let third = cups.pop_front().unwrap();

    let destination = calc_destination_optimized(
      current_cup,
      vec![first, second, third],
      max_size
    );
    /*
    The crab places the cups it just picked up so that they are
    immediately clockwise of the destination cup. They keep the same order
    as when they were picked up.
    The crab selects a new current cup: the cup which is immediately
    clockwise of the current cup.
    */
    while let Some(i) = cups.pop_front() {
      cups.push_back(i);
      if i == current_cup {
        break;
      } else if i == destination {
        cups.push_back(first);
        cups.push_back(second);
        cups.push_back(third);
      }
    }
  }
  /*
  After the crab is done, what order will the cups be in? Starting
  after the cup labeled 1, collect the other cups' labels clockwise
  into a single string with no extra characters; each number except
  1 should appear exactly once.
  */
  while let Some(i) = cups.pop_front() {
    if i == 1 {
      break;
    }
    cups.push_back(i);
  }

  cups.iter()
    .map(|v| std::char::from_digit(*v as u32, 10).unwrap())
    .collect::<String>()
    .parse::<usize>().unwrap()
}

fn main() {
  // let args: Vec<String> = env::args().collect();
  // 
  let input = parse_input(&mut std::io::stdin().lock().lines().map(|r| r.unwrap()));
  println!("{:?}", solve(&input, 0, 100, 9));
  println!("{:?}", solve(&input, 1000000, 10000000, 1000000));
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_samples() {
      
    }

}