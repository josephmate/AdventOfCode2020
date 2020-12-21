// .lines()
use std::io::prelude::*;
use std::collections::HashSet;
use std::collections::HashMap;
use std::collections::VecDeque;
use std::env;
use std::cmp::Ordering;


/*
              y
              |     z
              |    /
              |   / 
              |  /
              | /
     __________/__________ x
             /|
            / |
           /  |
          /   |
              |

   x     y     z
   0     0     0
   0     0    +1
   0     0    -1
        +1     0
        +1    +1
        +1    -1
        -1     0
        -1    +1
        -1    -1
  +1     0     0
  +1     0    +1
  +1     0    -1
  +1    +1     0
  +1    +1    +1
  +1    +1    -1
  +1    -1     0
  +1    -1    +1
  +1    -1    -1
  -1     0     0
  -1     0    +1
  -1     0    -1
  -1    +1     0
  -1    +1    +1
  -1    +1    -1
  -1    -1     0
  -1    -1    +1
  -1    -1    -1

*/

fn parse_input(
    lines: &mut dyn Iterator<Item = String>
) -> HashSet<(i64,i64,i64)> {
    let mut result = HashSet::new();

    for (x, line) in lines.enumerate() {
        for (y, c) in line.chars().enumerate() {
            // active (#) or inactive (.) state
            if c == '#' {
                result.insert((x as i64, y as i64, 0));
            }
        }
    }

    return result;
}

fn calc_neighbours(
    tuple: (i64, i64, i64)
) -> Vec<(i64, i64, i64)> {
    let (current_x, current_y, current_z) = tuple;
    let mut result = Vec::new();
    for x in -1 ..= 1 {
        for y in -1 ..= 1 {
            for z in -1 ..= 1 {
                if x != 0 || y != 0 || z != 0 {
                    result.push((current_x + x, current_y + y, current_z + z));
                }
            }
        }
    }
    return result;
}

fn calc_potentially_active_in_next_state(
    current: &HashSet<(i64,i64,i64)>
) -> HashSet<(i64,i64,i64)> {
    let mut result = HashSet::new();

    for coord in current {
        for neighbour in calc_neighbours(*coord) {
            result.insert(neighbour);
        }
    }

    return result;
}

fn calc_next_state(
    current: &HashSet<(i64,i64,i64)>
) -> HashSet<(i64,i64,i64)> {
    let mut next_actives = HashSet::new();

    for potentially_active in calc_potentially_active_in_next_state(&current) {
        //println!("  {:?}", potentially_active);
        let active_neighbours_count = calc_neighbours(potentially_active).iter()
            .filter(|neighbour| current.contains(neighbour))
            .count();
        //println!("    {:?}", active_neighbours_count);
        //println!("    {:?}", calc_neighbours(potentially_active));
        if current.contains(&potentially_active) {
            // active
            // If a cube is active and exactly 2 or 3 of its neighbors are also active, the cube remains active.
            // Otherwise, the cube becomes inactive.
            if active_neighbours_count >= 2 && active_neighbours_count <= 3 {
                next_actives.insert(potentially_active);
            }
        } else {
            // inactive
            // If a cube is inactive but exactly 3 of its neighbors are active, the cube becomes active.
            // Otherwise, the cube remains inactive.
            if active_neighbours_count == 3 {
                next_actives.insert(potentially_active);
            }
        }
    }

    return next_actives;
}

fn solve3d(
    input: &HashSet<(i64,i64,i64)>
) -> usize {
    let mut current_state = input.clone();

    for i in 0 .. 6 {
        //println!("{:?}", current_state);
        current_state = calc_next_state(&current_state);
    }

    return current_state.len();
}





fn calc_neighbours4d(
    tuple: (i64, i64, i64, i64)
) -> Vec<(i64, i64, i64, i64)> {
    let (current_x, current_y, current_z, current_w) = tuple;
    let mut result = Vec::new();
    for x in -1 ..= 1 {
        for y in -1 ..= 1 {
            for z in -1 ..= 1 {
                for w in -1 ..= 1 {
                    if x != 0 || y != 0 || z != 0 || w != 0 {
                        result.push((
                            current_x + x,
                            current_y + y,
                            current_z + z,
                            current_w + w));
                    }
                }
            }
        }
    }
    return result;
}

fn calc_potentially_active_in_next_state4d(
    current: &HashSet<(i64,i64,i64,i64)>
) -> HashSet<(i64,i64,i64,i64)> {
    let mut result = HashSet::new();

    for coord in current {
        for neighbour in calc_neighbours4d(*coord) {
            result.insert(neighbour);
        }
    }

    return result;
}

fn calc_next_state4d(
    current: &HashSet<(i64,i64,i64,i64)>
) -> HashSet<(i64,i64,i64,i64)> {
    let mut next_actives = HashSet::new();

    for potentially_active in calc_potentially_active_in_next_state4d(&current) {
        //println!("  {:?}", potentially_active);
        let active_neighbours_count = calc_neighbours4d(potentially_active).iter()
            .filter(|neighbour| current.contains(neighbour))
            .count();
        //println!("    {:?}", active_neighbours_count);
        //println!("    {:?}", calc_neighbours(potentially_active));
        if current.contains(&potentially_active) {
            // active
            // If a cube is active and exactly 2 or 3 of its neighbors are also active, the cube remains active.
            // Otherwise, the cube becomes inactive.
            if active_neighbours_count >= 2 && active_neighbours_count <= 3 {
                next_actives.insert(potentially_active);
            }
        } else {
            // inactive
            // If a cube is inactive but exactly 3 of its neighbors are active, the cube becomes active.
            // Otherwise, the cube remains inactive.
            if active_neighbours_count == 3 {
                next_actives.insert(potentially_active);
            }
        }
    }

    return next_actives;
}

fn to_4d(
    input: &HashSet<(i64,i64,i64)>
) -> HashSet<(i64,i64,i64,i64)> {
    let mut result = HashSet::new();
    for (x,y,z) in input {
        result.insert((*x,*y,*z,0));
    }
    return result;
}

fn solve4d(
    input: &HashSet<(i64,i64,i64,i64)>
) -> usize {
    let mut current_state = input.clone();

    for i in 0 .. 6 {
        //println!("{:?}", current_state);
        current_state = calc_next_state4d(&current_state);
    }

    return current_state.len();
}

fn main() {
    // let args: Vec<String> = env::args().collect();
    
    let active_coords = parse_input(&mut std::io::stdin().lock().lines()
        .map(|line_result| line_result.unwrap()));

    println!("{}", solve3d(&active_coords));
    println!("{}", solve4d(&to_4d(&active_coords)));
}

