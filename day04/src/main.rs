// .lines()
use std::io::prelude::*;
use std::vec::Vec;

fn parse_row(line: String) -> Vec<bool> {
    return line.chars()
        .map(|c| match c {
            '#' => true,
            _ => false // assume there are only '#' and '.' characters in the input
        })
        .collect();
}

fn try_slope(
    tree_positions : &Vec<Vec<bool>>,
    down_distance : usize,
    right_distance : usize
) -> i64
{
    let map_width = tree_positions.iter().next().unwrap().len();
    let mut current_row = 0;
    let mut current_col = 0;
    let mut trees_hit = 0;
    while current_row < tree_positions.len() {
        if tree_positions[current_row][current_col] {
            trees_hit += 1;
        }

        current_row += down_distance;
        current_col = (current_col + right_distance) % map_width;
    }

    return trees_hit;
}

fn main() {
    let tree_positions : Vec<Vec<bool>> = 
        std::io::stdin().lock().lines()
            .map(|l| parse_row(l.unwrap()))
            .collect();

    let trees_hit = try_slope(&tree_positions, 1, 3);
    println!("{}", trees_hit);

    // Right 3, down 1. (This is the slope you already checked.)
    // Right 1, down 1.
    // Right 5, down 1.
    // Right 7, down 1.
    // Right 1, down 2.
    // What do you get if you multiply together the number of trees encountered on each of the listed slopes?

    println!("{}", (
        trees_hit
        * try_slope(&tree_positions, 1, 1) 
        * try_slope(&tree_positions, 1, 5) 
        * try_slope(&tree_positions, 1, 7) 
        * try_slope(&tree_positions, 2, 1) 
    ));
}
