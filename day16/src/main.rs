// .lines()
use std::io::prelude::*;
use std::collections::HashSet;
use std::collections::HashMap;
use std::collections::VecDeque;
use std::env;
use std::cmp::Ordering;

/*
class: 1-3 or 5-7
row: 6-11 or 33-44
seat: 13-40 or 45-50

your ticket:
7,1,14

nearby tickets:
7,3,47
40,4,50
55,2,20
38,6,12
*/
fn parse(
    lines:  & mut dyn Iterator<Item = String>
) -> (Vec<(usize, usize, usize, usize)>, Vec<usize>, Vec<Vec<usize>>) {
    let mut validations = Vec::new();
    let mut current_line = lines.next().unwrap();
    while !current_line.is_empty() {
        //                0            1
        // departure location: 40-152 or 161-969
        let mut tokens = current_line.split(": ");
        tokens.next(); // field_name unused
        let mut range_tokens = tokens.next().unwrap().split(" or ");
        let mut first_range_tokens = range_tokens.next().unwrap().split("-");
        let first_range_lower = first_range_tokens.next().unwrap().parse::<usize>().unwrap();
        let first_range_upper = first_range_tokens.next().unwrap().parse::<usize>().unwrap();
        tokens.next(); // or unused
        let mut second_range_tokens = range_tokens.next().unwrap().split("-");
        let second_range_lower = second_range_tokens.next().unwrap().parse::<usize>().unwrap();
        let second_range_upper = second_range_tokens.next().unwrap().parse::<usize>().unwrap();
        validations.push((first_range_lower, first_range_upper, second_range_lower, second_range_upper));

        current_line = lines.next().unwrap();
    }

    lines.next(); // your ticket:
    let my_ticket = lines.next().unwrap().split(",")
        .map(|v| v.parse::<usize>().unwrap())
        .collect();

    lines.next(); // blank
    lines.next(); // nearby tickets
    let mut nearby_tickets = Vec::new();
    while let Some(line) = &lines.next() {
        nearby_tickets.push(line.split(",")
            .map(|v| v.parse::<usize>().unwrap())
            .collect());
    }

    return (validations, my_ticket, nearby_tickets);
}

fn potentially_valid_field(
    ticket_field: usize,
    validations: &Vec<(usize, usize, usize, usize)>
) -> bool {
    for validation in validations {
        let (first_lower, first_upper, second_lower, second_upper) = validation;
        if (*first_lower <= ticket_field && ticket_field <= *first_upper)
            || (*second_lower <= ticket_field && ticket_field <= *second_upper)
        {
            return true;
        }
    }

    return false;
}

fn validate_tickets(
    validations: &Vec<(usize, usize, usize, usize)>,
    nearby_tickets: &Vec<Vec<usize>>
) -> usize {
    let mut invalidation_sum = 0;

    for nearby_ticket in nearby_tickets.iter() {
        for ticket_field in nearby_ticket {
            if !potentially_valid_field(*ticket_field, validations) {
                invalidation_sum += ticket_field;
            }
        }
    }

    return invalidation_sum;
}

fn potentially_valid_ticket(
    nearby_ticket: &Vec<usize>,
    validations: &Vec<(usize, usize, usize, usize)>
) -> bool {
    for ticket_field in nearby_ticket {
        if !potentially_valid_field(*ticket_field, validations) {
            return false;
        }
    }

    return true;
}

// if there is any ticket that does not validate for that posn,
// then that posn is not valid
fn field_valid_in_all_tickets(
    validation: (usize, usize, usize, usize),
    field_idx: usize,
    posn: usize,
    nearby_tickets: &Vec<Vec<usize>>
) -> Option<(usize, usize, usize)> {
    let (first_lower, first_upper, second_lower, second_upper) = validation; 
    for nearby_ticket in nearby_tickets {
        let ticket_field = nearby_ticket[posn];
        let (first_lower, first_upper, second_lower, second_upper) = validation;
        if !(first_lower <= ticket_field && ticket_field <= first_upper)
            && !(second_lower <= ticket_field && ticket_field <= second_upper)
        {
            return Some((field_idx, posn, ticket_field));
        }
    }
    return None;
}

fn solve_fields(
    validations: &Vec<(usize, usize, usize, usize)>,
    nearby_tickets: &Vec<Vec<usize>>
) -> Vec<usize> {
    let mut valid_tickets: Vec<Vec<usize>> = Vec::new();
    for nearby_ticket in nearby_tickets {
        if potentially_valid_ticket(nearby_ticket, validations) {
            valid_tickets.push(nearby_ticket.to_vec());
        }
    }

    // map of field to 
    let mut potential_fields_to_positions = HashMap::new();
    for field_idx in 0 .. validations.len() {
        let validation = validations[field_idx];
        let mut positions = HashSet::new();
        for posn in 0 .. validations.len() {
            match field_valid_in_all_tickets(validation, field_idx, posn, &valid_tickets)  {
                Some(tuple) => {
                    //println!("removed field posn value: {:?}", tuple);
                },
                None => {
                    positions.insert(posn);
                },
            }
        }
        potential_fields_to_positions.insert(field_idx, positions);
    }

    let mut field_to_posn = HashMap::new();
    // if the field only has one position to go into then that's the one
    // loop until map is empty
    while !potential_fields_to_positions.is_empty() {
        //println!("{:?}", potential_fields_to_positions);
        let keys: Vec<usize> = potential_fields_to_positions.keys().cloned().collect();
        for k in keys {
            let potential_posns = potential_fields_to_positions[&k].len();
            let potential_posn = potential_fields_to_positions[&k].iter().cloned().next().unwrap();
            if potential_posns == 1 {
                field_to_posn.insert(k, potential_posn);
                potential_fields_to_positions.remove(&k);
                for value in potential_fields_to_positions.values_mut() {
                    value.remove(&potential_posn);
                }
            }
        }
    }

    let mut result = Vec::new();
    for field_idx in 0 .. validations.len() {
        result.push(field_to_posn[&field_idx]);
    }

    return result;
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let part1 = args[1].parse::<bool>().unwrap();
    let part2 = args[2].parse::<bool>().unwrap();
    let part3 = args[3].parse::<bool>().unwrap();

    let (validations, my_ticket, nearby_tickets) = 
        parse(& mut std::io::stdin().lock().lines()
            .map(|line_result| line_result.unwrap())
        );
    
    if part1 {
        println!("{}", validate_tickets(&validations, &nearby_tickets));
    }
    if part2 {
        let field_mapping = solve_fields(&validations, &nearby_tickets);
        println!("{:?}", field_mapping);
        if part3 {
            let mut my_ticket_product = 1;
            for i in 0 .. 6 {
                my_ticket_product *= my_ticket[field_mapping[i]];
            }
            println!("{:?}", my_ticket_product);
        }
    }
    
}

