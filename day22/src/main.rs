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

  calc_score(&deck)
}

fn calc_score(deck: &VecDeque<usize>) -> usize {
  let num_cards = deck.len();
  (1..=num_cards).rev()
    .zip( deck.iter() )
    .map(|(a, b)| a * b)
    .sum()
}

fn solve_part2(
  player1: &[usize],
  player2: &[usize],
) -> (bool, usize) {

  let mut visited: HashSet<(Vec<usize>,Vec<usize>)> = HashSet::new();

  let mut deck1 = player1.iter()
    .copied()
    .collect::<VecDeque<usize>>();
  let mut deck2 = player2.iter()
    .copied()
    .collect::<VecDeque<usize>>();

  while !deck1.is_empty() && !deck2.is_empty() {
    /*
    Before either player deals a card, if there was a previous round in this game that had
    exactly the same cards in the same order in the same players' decks, the game instantly
    ends in a win for player 1. Previous rounds from other games are not considered. (This
      prevents infinite games of Recursive Combat, which everyone agrees is a bad idea.)
    */
    let deck1_copy = deck1.iter().copied().collect::<Vec<usize>>();
    let deck2_copy = deck2.iter().copied().collect::<Vec<usize>>();
    if visited.contains(&(deck1_copy.to_vec(), deck2_copy.to_vec()))
    {
      return (true, calc_score(&deck1));
    }
    visited.insert((deck1_copy, deck2_copy));


    /*
    Otherwise, this round's cards must be in a new configuration; the players begin the
    round by each drawing the top card of their deck as normal.
    */
    let value1 = deck1.pop_front().unwrap();
    let value2 = deck2.pop_front().unwrap();
    if value1 <= deck1.len() && value2 <= deck2.len() {
      /*
      If both players have at least as many cards remaining in their deck as the value of
      the card they just drew, the winner of the round is determined by playing a new
      game of Recursive Combat (see below).
      */
      let (recurse_result, _) = solve_part2(
        &deck1.iter().take(value1).copied().collect::<Vec<usize>>(),
        &deck2.iter().take(value2).copied().collect::<Vec<usize>>());
      if recurse_result {
        deck1.push_back(value1);
        deck1.push_back(value2);
      } else {
        deck2.push_back(value2);
        deck2.push_back(value1);
      }
    } else {
      /*
      Otherwise, at least one player must not have enough cards left in their deck to
      recurse; the winner of the round is the player with the higher-value card.
      */
      if value1 > value2 {
        deck1.push_back(value1);
        deck1.push_back(value2);
      } else {
        deck2.push_back(value2);
        deck2.push_back(value1);
      }
    }
  }
  
  if deck1.is_empty() {
    (false, calc_score(&deck2))
  } else {
    (true, calc_score(&deck1))
  }
}

fn main() {
  // let args: Vec<String> = env::args().collect();
  // &mut std::io::stdin().lock().lines()
  let (player1, player2) = parse_input(&mut std::io::stdin().lock().lines().map(|sr| sr.unwrap()));
  
  println!("{}", solve_part1(&player1, &player2));
  let (_did_player1_win, score) = solve_part2(&player1, &player2);
  println!("{}", score);
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_samples() {
      
    }

}