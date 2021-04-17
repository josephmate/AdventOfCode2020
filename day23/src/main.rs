#![allow(unused_imports)]
// .lines()
use std::io::prelude::*;
use std::collections::HashSet;
use std::collections::HashMap;
use std::collections::VecDeque;
use std::env;
use std::cmp::Ordering;
use std::rc::Rc;

#[derive(Debug,PartialEq)]
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

/**
 * Assumes input has 10 elements and extras >= 11
 */
fn setup_cups(
  input: &[usize],
  extras: usize
) -> HashMap<usize, CupPointer> {
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

  if extras < 10 {
    cups.get_mut(&input[8]).unwrap().next_cup = input[0];
    cups.get_mut(&input[0]).unwrap().prev_cup = input[8];
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

  cups
}

fn solve(
  input: &[usize],
  extras: usize,
  iterations: usize,
  max_size: usize,
  concat_result: bool
) -> usize {
  let mut cups = setup_cups(input, extras);

  let mut current_cup = input[0];
  for _ in 0..iterations {
    /*
    The crab picks up the three cups that are immediately clockwise of the current cup.
    They are removed from the circle; cup spacing is adjusted as necessary to maintain the circle.
    */
    let first = cups[&current_cup].next_cup;
    let second = cups[&first].next_cup;
    let third = cups[&second].next_cup;
    //println!("1 2 3: {}, {}, {}", first, second, third);
    // connect the current up to the one after the third cup since they have been removed
    cups.get_mut(&current_cup).unwrap().next_cup = cups[&third].next_cup;

    let destination = calc_destination_optimized(
      current_cup,
      vec![first, second, third],
      max_size
    );
    //println!("destination: {}", destination);

    /*
    The crab places the cups it just picked up so that they are
    immediately clockwise of the destination cup. They keep the same order
    as when they were picked up.
    */
    let next_to_destination = cups[&destination].next_cup;
    cups.get_mut(&destination).unwrap().next_cup = first;
    cups.get_mut(&first).unwrap().prev_cup = destination;
    cups.get_mut(&third).unwrap().next_cup = next_to_destination;
    //println!("next_to_destination: {}", next_to_destination);
    cups.get_mut(&next_to_destination).unwrap().prev_cup = third;

    /*
    The crab selects a new current cup: the cup which is immediately
    clockwise of the current cup.
    */
    current_cup = cups[&current_cup].next_cup;
  }

  let mut result = 0;
  if concat_result {
    /*
    After the crab is done, what order will the cups be in? Starting
    after the cup labeled 1, collect the other cups' labels clockwise
    into a single string with no extra characters; each number except
    1 should appear exactly once.
    */
    let mut current_gathering_cup = cups[&1].next_cup;
    while current_gathering_cup != 1 {
      result *= 10;
      result += current_gathering_cup;
      current_gathering_cup = cups[&current_gathering_cup].next_cup;
    }
  } else {
    /*
    The crab is going to hide your stars - one each - under the two cups that will end up immediately clockwise of cup 1.
    You can have them if you predict what the labels on those cups will be when the crab is finished.
    Determine which two cups will end up immediately clockwise of cup 1.
    What do you get if you multiply their labels together?
    */
    let next_to_one = cups[&1].next_cup;
    let second_next_to_one = cups[&next_to_one].next_cup;
    result = next_to_one * second_next_to_one;
  }

  result
}

fn main() {
  let input = parse_input(&mut std::io::stdin().lock().lines().map(|r| r.unwrap()));
  println!("{:?}", solve(&input, 0, 100, 9, true));
  println!("{:?}", solve(&input, 1000000, 10000000, 1000000, false));
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_map_cups() {
      let input = [3,8,9,1,2,5,4,6,7];
      let actual_cups = setup_cups(&input, 12);
      let mut expected_cups: HashMap<usize, CupPointer> = HashMap::new();
      expected_cups.insert(3, CupPointer{
        next_cup: 8,
        prev_cup: 12
      });
      expected_cups.insert(8, CupPointer{
        next_cup: 9,
        prev_cup: 3
      });
      expected_cups.insert(9, CupPointer{
        next_cup: 1,
        prev_cup: 8
      });
      expected_cups.insert(1, CupPointer{
        next_cup: 2,
        prev_cup: 9
      });
      expected_cups.insert(2, CupPointer{
        next_cup: 5,
        prev_cup: 1
      });
      expected_cups.insert(5, CupPointer{
        next_cup: 4,
        prev_cup: 2
      });
      expected_cups.insert(4, CupPointer{
        next_cup: 6,
        prev_cup: 5
      });
      expected_cups.insert(6, CupPointer{
        next_cup: 7,
        prev_cup: 4
      });
      expected_cups.insert(7, CupPointer{
        next_cup: 10,
        prev_cup: 6
      });
      expected_cups.insert(10, CupPointer{
        next_cup: 11,
        prev_cup: 7
      });
      expected_cups.insert(11, CupPointer{
        next_cup: 12,
        prev_cup: 10
      });
      expected_cups.insert(12, CupPointer{
        next_cup: 3,
        prev_cup: 11
      });
      for i in 1..=12 {
        assert_eq!(actual_cups[&i], expected_cups[&i]);
      }
    }

}