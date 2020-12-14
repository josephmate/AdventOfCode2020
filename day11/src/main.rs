// .lines()
use std::io::prelude::*;
use std::collections::HashSet;
use std::collections::HashMap;
use std::collections::VecDeque;
use std::env;



fn get_bounds(grid: &Vec<Vec<char>>) -> (usize,usize) {
    return (grid.len(), grid.iter().next().unwrap().len());
}

fn get_first_visible_seat(
    row: i64,
    col: i64,
    rows: i64,
    cols: i64,
    r_delta: i64,
    c_delta: i64,
    grid: &Vec<Vec<char>>
) -> Option<char> {
    let mut r_translate = r_delta.clone();
    let mut c_translate = c_delta.clone();

    while  (row + r_translate) >= 0 && (row + r_translate) < rows
        && (col + c_translate) >= 0 && (col + c_translate) < cols
    {
        let state = grid[(row + r_translate) as usize][(col + c_translate) as usize];
        match state {
            'L'  | '#' =>  return Some(state),
            _ => (),
        };
        r_translate += r_delta;
        c_translate += c_delta;
    }
    return None;
}

fn get_surrounding_visible (
    row: usize,
    col: usize,
    grid: &Vec<Vec<char>>
) -> Vec<char> {
    let mut surrounding = Vec::new();
    let (rows, cols) = get_bounds(grid);
    let row = row as i64;
    let col = col as i64;
    let rows = rows as i64;
    let cols = cols as i64;

    for r_delta in -1 ..= 1 {
        let r_delta = r_delta as i64;
        for c_delta in -1 ..= 1 {
            let c_delta = c_delta as i64;
            if r_delta != 0 || c_delta != 0
            {
                match get_first_visible_seat(row, col, rows, cols, r_delta, c_delta, grid) {
                    Some(c) => surrounding.push(c),
                    _ => (),
                };
            }
        }
    }

    return surrounding;
}

fn get_surrounding_simple (
    row: usize,
    col: usize,
    grid: &Vec<Vec<char>>
) -> Vec<char> {
    let mut surrounding = Vec::new();
    let (rows, cols) = get_bounds(grid);
    let row = row as i64;
    let col = col as i64;
    let rows = rows as i64;
    let cols = cols as i64;

    for r_delta in -1 ..= 1 {
        let r_delta = r_delta as i64;
        for c_delta in -1 ..= 1 {
            let c_delta = c_delta as i64;
            if (r_delta != 0 || c_delta != 0)
                && (row + r_delta) >= 0 && (row + r_delta) < rows
                && (col + c_delta) >= 0 && (col + c_delta) < cols
            {
                surrounding.push(grid[(row + r_delta) as usize][(col + c_delta) as usize]);
            }
        }
    }

    return surrounding;
}

fn print_grid(
    grid: &Vec<Vec<char>>,
) {
    let (rows, cols) = get_bounds(grid);
    for r in 0..rows {
        for c in 0..cols {
            print!("{}", grid[r][c]);
        }
        println!("");
    }
    println!("");
}

// Each position is either floor (.)
// an empty seat (L),
// or an occupied seat (#)

//All decisions are based on the number of occupied seats adjacent to a given seat
// (one of the eight positions immediately up, down, left, right, or diagonal from the seat).
//The following rules are applied to every seat simultaneously:
fn calc_next_state(
    current_grid: &Vec<Vec<char>>,
    too_many_occuppied: usize,
    get_surrounding: &dyn Fn(usize,usize,&Vec<Vec<char>>) -> Vec<char>
) -> Vec<Vec<char>> {
    let mut next_grid = Vec::new();
    let (rows, cols) = get_bounds(current_grid);

    for r in 0..rows {
        let mut next_row = Vec::new();
        for c in 0..cols {
            let new_state : char = match current_grid[r][c] {
                'L' => {
                    //    If a seat is empty (L) and there are no occupied seats adjacent to it, the seat becomes occupied.
                    let occupied_count = get_surrounding(r,c, current_grid).iter()
                        .filter(|c| **c == '#')
                        .count();
                    if occupied_count == 0 {
                        '#'
                    } else {
                        //    Otherwise, the seat's state does not change.
                        'L'
                    }
                },
                '#' => {
                    //    If a seat is occupied (#) and four or more seats adjacent to it are also occupied, the seat becomes empty.
                    let occupied_count = get_surrounding(r,c, current_grid).iter()
                        .filter(|c| **c == '#')
                        .count();
                    if occupied_count >= too_many_occuppied {
                        'L'
                    } else {
                        //    Otherwise, the seat's state does not change.
                        '#'
                    }
                    //    Otherwise, the seat's state does not change.
                },
                //Floor (.) never changes; seats don't move, and nobody sits on the floor.
                _ => '.'
            };
            next_row.push(new_state);
        }
        next_grid.push(next_row);
    }

    //print_grid(&next_grid);
    return next_grid;
}

fn is_same_grid(a: &Vec<Vec<char>>, b: &Vec<Vec<char>>) -> bool {
    let (rows, cols) = get_bounds(a);

    for r in 0..rows {
        for c in 0..cols {
            if a[r][c] != b[r][c] {
                return false;
            }
        }
    }

    return true;
}

fn find_steady_seats(
    seat_grid: Vec<Vec<char>>,
    too_many_occuppied: usize,
    get_surrounding: &dyn Fn(usize,usize,&Vec<Vec<char>>) -> Vec<char>
) -> usize {
    let mut current_grid = seat_grid;
    let mut next_grid = calc_next_state(&current_grid, too_many_occuppied, get_surrounding);
    while !is_same_grid(&current_grid, &next_grid) {
        current_grid = next_grid;
        next_grid = calc_next_state(&current_grid, too_many_occuppied, get_surrounding);
    }

    return current_grid.iter()
        .flat_map(|grid| grid.iter())
        .filter(|c| **c == '#')
        .count();
}

fn main() {
    let seat_grid: Vec<Vec<char>> = std::io::stdin().lock().lines()
        .map(|line_result| line_result.unwrap().chars().collect())
        .collect();
    let seat_grid_copy = seat_grid.to_vec();

    let steady_seat_count = find_steady_seats(seat_grid, 4, &get_surrounding_simple);
    println!("{}", steady_seat_count);

    let steady_seat_count = find_steady_seats(seat_grid_copy, 5, &get_surrounding_visible);
    println!("{}", steady_seat_count);
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_right_dir() {
        assert_eq!(
            get_surrounding_visible(
                1, 1,
                &vec!(
                    ".............".chars().collect(),
                    ".L.L.#.#.#.#.".chars().collect(),
                    ".............".chars().collect()
                )
            ),
            vec!('L')
        );

        assert_eq!(
            get_surrounding_visible(
                1, 3,
                &vec!(
                    ".............".chars().collect(),
                    ".L.L.#.#.#.#.".chars().collect(),
                    ".............".chars().collect()
                )
            ),
            vec!('L', '#')
        );
    }

    fn test_none() {
        assert_eq!(
            get_surrounding_visible(
                3, 3,
                &vec!(
                    ".##.##.".chars().collect(),
                    "#.#.#.#".chars().collect(),
                    "##...##".chars().collect(),
                    "...L...".chars().collect(),
                    "##...##".chars().collect(),
                    "#.#.#.#".chars().collect(),
                    ".##.##.".chars().collect()
                )
            ),
            vec!()
        );

    }

    fn test_all() {

        assert_eq!(
            get_surrounding_visible(
                4, 3,
                &vec!(
                    ".......#.".chars().collect(),
                    "...#.....".chars().collect(),
                    ".#.......".chars().collect(),
                    ".........".chars().collect(),
                    "..#L....#".chars().collect(),
                    "....#....".chars().collect(),
                    ".........".chars().collect(),
                    "#........".chars().collect(),
                    "...#.....".chars().collect()
                )
            ),
            vec!('#', '#', '#', '#', '#', '#', '#', '#')
        );



    }
}