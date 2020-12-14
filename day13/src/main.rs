// .lines()
use std::io::prelude::*;
use std::collections::HashSet;
use std::collections::HashMap;
use std::collections::VecDeque;
use std::env;

fn min_time(
    arrival_time: i64,
    bus_ids: Vec<i64>
) -> i64 {
    let mut min_id = -1;
    let mut min_time_left = std::i64::MAX;

    for bus_id in bus_ids {
        let modulus = arrival_time % bus_id;
        if modulus == 0 {
            return 0;
        }

        let wait_time = bus_id - modulus;
        if wait_time < min_time_left {
            min_id = bus_id;
            min_time_left = wait_time;
        }
    }
    // What is the ID of the earliest bus you can take to the airport
    // multiplied by the number of minutes you'll need to wait for
    // that bus?
    return min_id * min_time_left;
}

/*
0 1  2 3 4  5 6  7
7,13,x,x,59,x,31,19
x % 7 == 0
(x + 1) % 13 == 0
(x + 4) % 59 == 0
(x + 6) % 31 == 0
(x + 7) % 19 == 0
smallest such x


1068781 % 7 = 0
(1068781 + 1) % 13
(1068781 + 4) % 59
(1068781 + 6) % 31
(1068781 + 7) % 19
*/

fn main() {
    let stdin = std::io::stdin();
    let mut line_iter = stdin.lock().lines();
    let arrival_time = line_iter.next().unwrap().unwrap().parse::<i64>().unwrap();
    let bus_ids: Vec<Option<i64>> = line_iter.next().unwrap().unwrap().split(",")
        .map(|token| {
            if token.eq("x") {
                None
            } else {
                Some(token.parse::<i64>().unwrap())
            }
        })
        .collect();
    let bus_id_copy = bus_ids.to_vec();
    let bus_ids = bus_ids.iter()
        .filter(|opt| opt.is_some())
        .map(|opt| opt.unwrap())
        .collect();
    
    println!("{}", min_time(arrival_time, bus_ids));

}