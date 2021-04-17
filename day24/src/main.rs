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

fn count_odd_flips(coords: &mut dyn Iterator<Item = (i64,i64)>) -> usize {
  let mut flip_counts: HashMap<(i64,i64), usize> = HashMap::new();
  while let Some(coord) = (*coords).next() {
    let count = flip_counts.entry(coord).or_insert(0);
    *count += 1;
  }

  flip_counts.values()
    .filter(|count| *count % 2 == 1)
    .count()
}

fn main() {
  // let args: Vec<String> = env::args().collect();
  let odd_flips = count_odd_flips(
    &mut std::io::stdin().lock().lines()
      .map(|line| line.unwrap())
      .map(path_to_coord)
  );
  println!("{}", odd_flips);
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_samples() {
      
    }

}