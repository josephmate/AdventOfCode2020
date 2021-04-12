#![allow(unused_imports)]
// .lines()
use std::io::prelude::*;
use std::collections::HashSet;
use std::collections::HashMap;
use std::collections::VecDeque;
use std::env;
use std::cmp::Ordering;
use std::rc::Rc;


struct CupPointer {
  next_cup: usize,
  prev_cup: usize,
}

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
) -> String {

  let mut cups = HashMap::new();
  for i in 0..input.len() {
    if i == 0 {
      cups.insert(input[i],
        CupPointer {
          prev_cup: 0,
          next_cup: input[i+1],
        }
      );
    } else if i == input.len() - 1 {
      cups.insert(input[i],
        CupPointer {
          prev_cup: input[i-1],
          next_cup: 0,
        }
      );
    } else {
      cups.insert(input[i],
        CupPointer {
          prev_cup: input[i-1],
          next_cup: input[i+1],
        }
      );
    }
  }

  for i in 10..=extras {
    if i == 10 {
      cups.get_mut(&input[input.len()-1]).unwrap().next_cup = i;
      cups.insert(i,
        CupPointer{
          prev_cup: input[input.len()-1],
          next_cup: i+1
        }
      );
    } else if i == extras {
      cups.get_mut(&input[0]).unwrap().prev_cup = i;
      cups.insert(i,
        CupPointer{
          prev_cup: i-1,
          next_cup: input[0]
        }
      );
    } else {
      cups.insert(i,
        CupPointer{
          prev_cup: i-1,
          next_cup: i+1
        }
      );
    }
  }

  let mut current_cup = input[0];
  for _ in 0..iterations {
    /*
    The crab picks up the three cups that are immediately clockwise of the current cup.
    They are removed from the circle; cup spacing is adjusted as necessary to maintain the circle.
    */
    let first = cups[&current_cup].next_cup;
    let second = cups[&first].next_cup;
    let third = cups[&second].next_cup;
    // connect the current up to the one after the third cup since they have been removed
    cups.get_mut(&current_cup).unwrap().next_cup = cups[&third].next_cup;

    let destination = calc_destination_optimized(
      current_cup,
      vec![first, second, third],
      max_size
    );

    /*
    The crab places the cups it just picked up so that they are
    immediately clockwise of the destination cup. They keep the same order
    as when they were picked up.
    */
    let next_to_destination = cups[&destination].next_cup;
    cups.get_mut(&destination).unwrap().next_cup = first;
    cups.get_mut(&first).unwrap().prev_cup = destination;
    cups.get_mut(&third).unwrap().next_cup = next_to_destination;
    cups.get_mut(&next_to_destination).unwrap().prev_cup = third;

    /*
    The crab selects a new current cup: the cup which is immediately
    clockwise of the current cup.
    */
    current_cup = cups[&current_cup].next_cup;
  }

  /*
  After the crab is done, what order will the cups be in? Starting
  after the cup labeled 1, collect the other cups' labels clockwise
  into a single string with no extra characters; each number except
  1 should appear exactly once.
  */
  let mut current_gathering_cup = cups[&1].next_cup;
  let mut  result = String::new();
  while current_gathering_cup != 1 {
    result.push(std::char::from_digit(current_gathering_cup as u32, 10).unwrap());
    current_gathering_cup = cups[&current_gathering_cup].next_cup;
  }

  result
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