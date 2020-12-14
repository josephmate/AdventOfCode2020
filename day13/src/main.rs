// .lines()
use std::io::prelude::*;
use std::collections::HashSet;
use std::collections::HashMap;
use std::collections::VecDeque;
use std::env;

fn parse_line(line: String) -> (char, i64) {
    return (line.chars().next().unwrap(),
        line[1..].parse::<i64>().unwrap());
}

fn abs(val: i64) -> i64 {
    if val >= 0 {
        return val;
    } else {
        return val * -1;
    }
}

fn run_instructions(
    instructions: Vec<(char, i64)>
) -> (i64, i64) {
    // directions ordered by turning right
    let directions = vec!('E', 'S', 'W', 'N');
    let mut current_direction_idx = 0;
    let mut current_x = 0; // +ve is E
    let mut current_y = 0; // +ve is N
    


    for (action_type, value) in instructions {
        match action_type {
            //Action N means to move north by the given value.
            'N' => current_y += value,
            //Action S means to move south by the given value.
            'S' => current_y -= value,
            //Action E means to move east by the given value.
            'E' => current_x += value,
            //Action W means to move west by the given value.
            'W' => current_x -= value,
            //Action L means to turn left the given number of degrees.
            'L' => {
                let new_direction = (current_direction_idx - (value/90)) % 4;
                if new_direction < 0 {
                    current_direction_idx = new_direction + 4;
                } else {
                    current_direction_idx = new_direction;
                }
            },
            //Action R means to turn right the given number of degrees.
            'R' => current_direction_idx = (current_direction_idx + (value/90)) % 4,
            //Action F means to move forward by the given value in the direction the ship is currently facing.
            _ => match directions[current_direction_idx as usize] {
                'N' => current_y += value,
                //Action S means to move south by the given value.
                'S' => current_y -= value,
                //Action E means to move east by the given value.
                'E' => current_x += value,
                //Action W means to move west by the given value.
                _ => current_x -= value,
            },
        };
    }
    
    return (current_x, current_y);
}





/*
      (-4, 10)
       d  |
          |
          |
          |         a (10, 4)  a
          |
----------*----------
          |
c         |
(-10, -4) |
          |
          |   b  (4, -10)
*/
fn rotate(
    x: i64,
    y: i64,
    degrees: i64,
    clockwise: bool
) -> (i64, i64) {
    let times = degrees / 90;

    let mut current_x = x;
    let mut current_y = y;
    
    for i in 0.. times {
        let temp_x = current_x;
        let temp_y = current_y;

        if clockwise {
            current_x = temp_y;
            current_y = -temp_x;
        } else {
            current_x = -temp_y;
            current_y = temp_x;
        }
    }

    return (current_x, current_y);
}

fn run_instructions_complex(
    instructions: Vec<(char, i64)>
) -> (i64, i64) {
    let mut current_direction_idx = 0;

    // ship's current position
    let mut current_x = 0; // +ve is E
    let mut current_y = 0; // +ve is N

    // The waypoint starts 10 units east and 1 unit north relative to the ship.
    // The waypoint is relative to the ship; that is, if the ship moves, the waypoint moves with it.
    let mut waypoint_x = 10; // +ve is E
    let mut waypoint_y = 1; // +ve is N
    
    for (action_type, value) in instructions {
        match action_type {
            // Action N means to move the waypoint north by the given value.
            'N' => waypoint_y += value,
            // Action S means to move the waypoint south by the given value.
            'S' => waypoint_y -= value,
            // Action E means to move the waypoint east by the given value.
            'E' => waypoint_x += value,
            // Action W means to move the waypoint west by the given value.
            'W' => waypoint_x -= value,
            // Action L means to rotate the waypoint around the ship left (counter-clockwise) the given number of degrees.
            'L' => {
                let (rotated_x, rotated_y) = rotate(waypoint_x, waypoint_y, value, false);
                waypoint_x = rotated_x;
                waypoint_y = rotated_y;
            },
            // Action R means to rotate the waypoint around the ship right (clockwise) the given number of degrees.
            'R' => {
                let (rotated_x, rotated_y) = rotate(waypoint_x, waypoint_y, value, true);
                waypoint_x = rotated_x;
                waypoint_y = rotated_y;
            },
            // Action F means to move forward to the waypoint a number of times equal to the given value.
            _ => {
                current_x += value*waypoint_x;
                current_y += value*waypoint_y;
            },
        };
    }
    
    return (current_x, current_y);
}

fn man((x, y): (i64, i64)) -> i64 {
    abs(x) + abs(y)
}

fn main() {
    let instructions: Vec<(char, i64)> = std::io::stdin().lock().lines()
        .map(|line_result| line_result.unwrap().chars().collect())
        .map(|line| parse_line(line))
        .collect();
    let instructions_clone = instructions.to_vec();

    println!("{}", man(run_instructions(instructions)));
    println!("{}", man(run_instructions_complex(instructions_clone)));
}



#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_forward() {
        assert_eq!(
            run_instructions(vec!(
                ('F', 1)
            )),
            (1, 0)
        );
    }

    #[test]
    fn test_r90() {
        assert_eq!(
            run_instructions(vec!(
                ('R', 90),
                ('F', 1),
            )),
            (0, -1)
        );
    }

    #[test]
    fn test_r180() {
        assert_eq!(
            run_instructions(vec!(
                ('R', 180),
                ('F', 1),
            )),
            (-1, 0)
        );
    }

    #[test]
    fn test_r270() {
        assert_eq!(
            run_instructions(vec!(
                ('R', 270),
                ('F', 1),
            )),
            (0, 1)
        );
    }

    #[test]
    fn test_l90() {
        assert_eq!(
            run_instructions(vec!(
                ('L', 90),
                ('F', 1),
            )),
            (0, 1)
        );
    }

    #[test]
    fn test_l180() {
        assert_eq!(
            run_instructions(vec!(
                ('L', 180),
                ('F', 1),
            )),
            (-1, 0)
        );
    }

    #[test]
    fn test_l270() {
        assert_eq!(
            run_instructions(vec!(
                ('L', 270),
                ('F', 1),
            )),
            (0, -1)
        );
    }
}