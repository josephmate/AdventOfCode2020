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

////////////////////////////////////////////// 
//     /\   /\   /\   /\   /\   /\   /\  
//    /  \ /  \ /  \ /  \ /  \ /  \ /  \
//   | -3 | -2 | -1 |  0 |  1 |  2 |  3 |
//   |    |    |    |    |    |    |    |
//   |  2 |  2 |  2 |  2 |  2 |  2 |  2 |
//    \  / \  / \  / \  / \  / \  / \  /
//     \/   \/   \/   \/   \/   \/   \/
//     | -5 | -3 | -1 |  1 |  3 |  5 |
//     |    |    |    |    |    |    | 
//     |  1 |  1 |  1 |  1 |  1 |  1 |
//     /\   /\   /\   /\   /\   /\   /\  
//    /  \ /  \ /  \ /  \ /  \ /  \ /  \
//   | -6 | -4 | -2 |  0 |  2 |  4 |  6 |
//   |    |    |    |    |    |    |    |
//   |  0 |  0 |  0 |  0 |  0 |  0 |  0 |
//    \  / \  / \  / \  / \  / \  / \  /
//     \/   \/   \/   \/   \/   \/   \/ 
//     | -5 | -3 | -1 |  1 |  3 |  5 |
//     |    |    |    |    |    |    | 
//     | -1 | -1 | -1 | -1 | -1 | -1 |
//     /\   /\   /\   /\   /\   /\   /\  
//    /  \ /  \ /  \ /  \ /  \ /  \ /  \
//   | -6 | -4 | -2 |  0 |  2 |  4 |  6 |
//   |    |    |    |    |    |    |    |
//   | -2 | -2 | -2 | -2 | -2 | -2 | -2 |
//    \  / \  / \  / \  / \  / \  / \  /
//     \/   \/   \/   \/   \/   \/   \/ 
//////////////////////////////////////////////
// These directions are given in your list, respectively, as e, se, sw, w, nw, and ne.
fn path_to_coord(path: String) -> (i64, i64) {
  let mut path_iter = path.chars();
  let mut x: i64 = 0;
  let mut y: i64 = 0;
  while let Some(direction_part) = path_iter.next() {
    match direction_part {
      'e' => {
        x += 2;
      },
      'w' => {
        x -= 2;
      },
      's' => {
        y -= 1;
        match path_iter.next() {
          Some('e') => {
            x += 1;
          },
          Some('w') => {
            x -= 1;
          },
          _=> {
            return (0,0);
          }
        }
      },
      'n' => {
        y += 1;
        match path_iter.next() {
          Some('e') => {
            x += 1;
          },
          Some('w') => {
            x -= 1;
          },
          _=> {
            return (0,0);
          }
        }
      },
      _=> {
        return (0,0);
      }
    }
  }

  (x, y)
}

fn group_by_coord(coords: &[(i64,i64)]) -> HashMap<(i64,i64), usize> {
  let mut flip_counts: HashMap<(i64,i64), usize> = HashMap::new();
  for coord in coords {
    let count = flip_counts.entry(*coord).or_insert(0);
    *count += 1;
  }
  flip_counts
}

fn count_odd_flips(coords: &[(i64,i64)]) -> usize {
  let flip_counts = group_by_coord(coords);
  flip_counts.values()
    .filter(|count| *count % 2 == 1)
    .count()
}

fn calc_neighbours((x,y): (i64,i64)) -> Vec<(i64,i64)> {
  return vec![
    (x+2,y  ), // e
    (x-2,y  ), // w
    (x+1,y+1), // ne
    (x-1,y+1), // nw
    (x+1,y-1), // se
    (x-1,y-1), // sw
  ];
}

fn calc_interesting_coords(current_state: &HashSet<(i64,i64)>) -> HashSet<(i64,i64)> {
  let mut interesting_coords = HashSet::new();
  for coord in current_state {
    interesting_coords.insert(*coord);
    for neighbour in calc_neighbours(*coord) {
      interesting_coords.insert(neighbour);
    }
  }
  interesting_coords
}

fn advance_state(current_state: HashSet<(i64,i64)>) -> HashSet<(i64,i64)> {
  let mut new_state = HashSet::new();

  let interesting_coords = calc_interesting_coords(&current_state);
  for interesting_coord in interesting_coords {
    let neighbours = calc_neighbours(interesting_coord);
    let odd_neighbour_count = neighbours.iter()
        .filter(|neighbour| current_state.contains(neighbour))
        .count();
    if current_state.contains(&interesting_coord) {
      // Any black tile with zero or more than 2 black tiles immediately adjacent to it is flipped to white.
      if odd_neighbour_count >= 1 && odd_neighbour_count <= 2 {
        new_state.insert(interesting_coord);
      }
    } else {
      // Any white tile with exactly 2 black tiles immediately adjacent to it is flipped to black.
      if odd_neighbour_count == 2 {
        new_state.insert(interesting_coord);
      }
    }
  }

  new_state
}

fn simulate_hexagonal_game_of_life(initial_coords: &[(i64,i64)]) -> usize {
  let mut current_state = HashSet::new();
  let flip_counts = group_by_coord(initial_coords);
  for ((x,y), count) in flip_counts {
    if count % 2 == 1 {
      current_state.insert((x,y));
    }
  }

  let mut most_recent_count = 0;
  for day in 1..=100 {
    most_recent_count = current_state.len();
    if (day >= 1 && day <= 10) || (day % 10 == 0) {
      println!("Day {}: {}", day, most_recent_count);
    }
    if day < 200 {
      current_state = advance_state(current_state);
    }
  }

  most_recent_count
}

fn main() {
  // let args: Vec<String> = env::args().collect();
  let day_one_coords: Vec<(i64,i64)> = std::io::stdin().lock().lines()
    .map(|line| line.unwrap())
    .map(path_to_coord)
    .collect();
  println!("{}", count_odd_flips(&day_one_coords));
  println!("{}", simulate_hexagonal_game_of_life(&day_one_coords));
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_samples() {
      
    }

}