// .lines()
use std::io::prelude::*;
use std::collections::HashSet;
use std::iter::FromIterator;

//0, 1
//0 (0+1)/ 2   0
//0 0

// 0 1
// (0+1)/2    1
// 0 1

// F means to take the lower half, keeping rows 0 through 63.
// L means to take the lower half, keeping columns 4 through 5.
// B means to take the upper half, keeping rows 32 through 63.
// R means to take the upper half, keeping columns 4 through 7.
fn bin_traverse(
    map_directions: &mut dyn Iterator<Item = char>,
    lower:i64,
    upper:i64
) -> i64 {
    match map_directions.next() {
        Some(next_step) => {
            match next_step {
                'F' | 'L' => return bin_traverse(map_directions, lower, (lower+upper)/2),
                _ => return bin_traverse(map_directions, (lower+upper)/2 + 1, upper),
            }
        },
        None => {
            return lower;
        },
    }
}

fn to_row_columm(map_directions: String) -> (i64, i64) {
    return (
        // FBFBBFF RLR
        bin_traverse(&mut map_directions.get(0..7).unwrap().chars(), 0, 127),
        bin_traverse(&mut map_directions.get(7..10).unwrap().chars(), 0, 7),
    );
}

//  multiply the row by 8 then add the column. In this example, the seat has ID 44 * 8 + 5 = 357.
fn to_seat_id((row, col): (i64,i64)) -> i64 {
    return row*8 + col;
}

fn main() {
    
    let seat_ids: HashSet<i64> = HashSet::from_iter(
        std::io::stdin().lock().lines()
            .map(|line| line.unwrap())
            .map(to_row_columm)
            .map(to_seat_id)
    );

    let max = seat_ids.iter()
        .max()
        .unwrap();
    println!("{}",  max);
    let min = seat_ids.iter()
        .min()
        .unwrap();
    
    for seat_id in *min..=*max {
        match seat_ids.get(&seat_id) {
            Some(_) => (),
            None => println!("{}", seat_id),
        }
    }

}
