// .lines()
use std::io::prelude::*;
use std::collections::HashSet;
use std::collections::HashMap;
use std::collections::VecDeque;
use std::env;
use std::cmp::Ordering;

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

fn is_special_time(
    time: i64,
    bus_ids: &Vec<Option<i64>>
) -> bool {
    for (idx, id) in bus_ids.iter().enumerate() {
        match id {
            Some(id) => {
                if (time + (idx as i64)) % id != 0 {
                    return false;
                }
            },
            None => (),
        }
    }

    return true;
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

0,7
    0 7 14 21 28 35 42 49 56 63 70 77
1,13
    13 - 1 = 12
       12    25   38    51     64  77
    (77+0) %  7 = 0
    (77+1) % 13 = 0
    77+(13*7) = 168
    (168 + 0) %  7 = 0
    (168 + 1) % 13 = 0
4,59
    77+(1)(13*7)   77+(2)(13*7)  77+(3)(13*7) ....
    168              259           350        441 532 623 714 805 896 987 1078 1169 1260 1351 1442           
    ---------------------------
    (168+4) % 59    (259+4) % 59  (350+4) % 59
        54              27            0
    (350+0) %  7 = 0 
    (350+1) % 13 = 0
    (350+4) % 59 = 0

offset = 0, factors {}, increment = 1
from 0 increment by 1 until x+0 % 7 = 0
0
0
offset = 0 , factors{7}, increment = 1*7 = 7
from 0 increment by 7 until x+1 % 13 = 0
0 7 14 21 28 35 42 49 56 63 70 77
77
offest = 77, factors{7,13}, increment = 1*7*13 = 91
from 77 increment by 91 until x+4 % 59 = 0
168              259           350
...


1068781 % 7 = 0
(1068781 + 1) % 13 = 0
(1068781 + 4) % 59 = 0
(1068781 + 6) % 31 = 0
(1068781 + 7) % 19 = 0
*/
fn special_time(
    bus_ids: Vec<Option<i64>>
) -> i64 {
    let (biggest_idx, biggest) = bus_ids.iter().enumerate()
        .filter(|(_, id)| id.is_some())
        .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap_or(Ordering::Equal))
        .map(|(idx, id)| (idx, id.unwrap()))
        .unwrap();
    let mut current_time = biggest - (biggest_idx as i64);

    while !is_special_time(current_time, &bus_ids) {
        current_time += biggest;
    }

    return current_time;
}


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
    println!("{}", special_time(bus_id_copy));
}