// .lines()
use std::io::prelude::*;
use std::collections::HashSet;
use std::collections::HashMap;
use std::collections::VecDeque;
use std::env;



fn get_bounds(grid: &Vec<Vec<char>>) -> (usize,usize) {
    return (grid.len(), grid.iter().next().unwrap().len());
}

fn get_surrounding_visible (
    row: usize,
    col: usize,
    grid: &Vec<Vec<char>>
) -> Vec<char> {
    return get_surrounding_simple(row, col, grid);
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

// Each position is either floor (.)
// an empty seat (L),
// or an occupied seat (#)

//All decisions are based on the number of occupied seats adjacent to a given seat
// (one of the eight positions immediately up, down, left, right, or diagonal from the seat).
//The following rules are applied to every seat simultaneously:
fn calc_next_state(
    current_grid: &Vec<Vec<char>>,
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
                    if occupied_count >= 4 {
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
    get_surrounding: &dyn Fn(usize,usize,&Vec<Vec<char>>) -> Vec<char>
) -> usize {
    let mut current_grid = seat_grid;
    let mut next_grid = calc_next_state(&current_grid, get_surrounding);
    while !is_same_grid(&current_grid, &next_grid) {
        current_grid = next_grid;
        next_grid = calc_next_state(&current_grid, get_surrounding);
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

    let steady_seat_count = find_steady_seats(seat_grid, &get_surrounding_simple);
    println!("{}", steady_seat_count);

    let steady_seat_count = find_steady_seats(seat_grid_copy, &get_surrounding_visible);
    println!("{}", steady_seat_count);
}
