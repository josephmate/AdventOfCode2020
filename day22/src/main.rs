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

fn parse_player(
  lines: &mut dyn Iterator<Item=String>
) -> Vec<usize> {
  lines.next(); // Ignore "Player 1:"" and "Player 2:" line
  let mut result = Vec::new();
  for line in lines {
    if line.is_empty() {
      break;
    }
    result.push(line.parse::<usize>().unwrap());
  }
  result
}

fn parse_input(
  lines: &mut dyn Iterator<Item=String>
) -> (Vec<usize>, Vec<usize>) {
  (parse_player(lines), parse_player(lines))
}

fn solve_part1(
  player1: &[usize],
  player2: &[usize],
) -> usize {
  let mut deck1 = player1.iter()
    .copied()
    .collect::<VecDeque<usize>>();
  let mut deck2 = player2.iter()
    .copied()
    .collect::<VecDeque<usize>>();

  while !deck1.is_empty() && !deck2.is_empty() {
    let value1 = deck1.pop_front().unwrap();
    let value2 = deck2.pop_front().unwrap();
    if value1 > value2 {
      deck1.push_back(value1);
      deck1.push_back(value2);
    } else {
      deck2.push_back(value2);
      deck2.push_back(value1);
    }
  }
  
  let deck = if deck1.is_empty() {
    deck2
  } else {
    deck1
  };
  let num_cards = deck.len();

  (1..=num_cards).rev()
    .zip( deck.iter() )
    .map(|(a, b)| a * b)
    .sum()
}

fn main() {
  // let args: Vec<String> = env::args().collect();
  // &mut std::io::stdin().lock().lines()
  let (player1, player2) = parse_input(&mut std::io::stdin().lock().lines().map(|sr| sr.unwrap()));

  println!("{}", solve_part1(&player1, &player2));
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_samples() {
      
    }

}