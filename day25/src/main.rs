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


fn calc_loop_size(public_key: usize) -> usize {

  for i in 0..=1000 {

  }

  // not found. max loop size not large enough.
  0
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

  let card_loop_size = calc_loop_size(card_public_key);
  let door_loop_size = calc_loop_size(door_public_key);
  println!("card_loop_size {}", card_loop_size);
  println!("door_loop_size {}", door_loop_size);
  println!("encryption_key {}", 0);
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_samples() {
      
    }

}