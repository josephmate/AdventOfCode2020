// .lines()
#[allow(unused_imports)]
use std::io::prelude::*;
#[allow(unused_imports)]
use std::collections::HashSet;
#[allow(unused_imports)]
use std::collections::HashMap;
#[allow(unused_imports)]
use std::collections::VecDeque;
#[allow(unused_imports)]
use std::env;
#[allow(unused_imports)]
use std::cmp::Ordering;


fn calc_loop_size(public_key: usize, subject_number: usize) -> usize {

  // The handshake used by the card and the door involves an operation that transforms a subject number.
  // To transform a subject number, start with the value 1.
  // Then, a number of times called the loop size, perform the following steps:
  // - Set the value to itself multiplied by the subject number.
  // - Set the value to the remainder after dividing the value by 20201227.

  let mut value = 1;
  if value == public_key {
    return 0;
  }
  for i in 1..=100000000 {
    value *= subject_number;
    value %= 20201227;
    if value == public_key {
      return i;
    }
  }

  // not found. max loop size not large enough.
  0
}

fn transform(subject_number: usize, loop_size: usize) -> usize {
  let mut value = 1;
  for _ in 1..=loop_size {
    value *= subject_number;
    value %= 20201227;
  }

  value
}

fn main() {
  // let args: Vec<String> = env::args().collect();

  let initial_subject_number = 7;
  println!("initial_subject_number {}", initial_subject_number);

  let input: Vec<usize> = std::io::stdin().lock().lines()
    .map(|result| result.unwrap())
    .map(|line| line.parse::<usize>())
    .map(|result| result.unwrap())
    .collect();
  let card_public_key = input[0];
  let door_public_key = input[1];
  println!("card_public_key {}", card_public_key);
  println!("door_public_key {}", door_public_key);

  let card_loop_size = calc_loop_size(card_public_key, initial_subject_number);
  let door_loop_size = calc_loop_size(door_public_key, initial_subject_number);
  println!("card_loop_size {}", card_loop_size);
  println!("door_loop_size {}", door_loop_size);
  println!("encryption_key {}", transform(door_public_key, card_loop_size));
  // makes the same
  //println!("encryption_key {}", transform(card_public_key, door_loop_size));
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_samples() {
      
    }

}