// .lines()
use std::io::prelude::*;
use std::collections::HashSet;
use std::collections::HashMap;
use std::collections::VecDeque;

fn parse_input(
    lines : &mut dyn Iterator<Item = Result<String, std::io::Error>>
) -> (
    HashMap<String, HashSet<String>>,
    HashMap<String, HashMap<String, i64>>
){
    let mut baggage_map = HashMap::new();
    let mut containment_map = HashMap::new();

    for line in lines {
        let line = line.unwrap();
        let mut a_contains_xyz_tokens = line.split(" contain ");
        let consuming_bag = a_contains_xyz_tokens.next().unwrap()
            .replace(" bags", "");

        let containing_bags_tokens = a_contains_xyz_tokens
            .next().unwrap()
            .split(", ");
        
        
        let mut baggage_counts = HashMap::new();
        for containing_bag_desc_str in containing_bags_tokens {
            if !containing_bag_desc_str.starts_with("no other bags") {
                let mut containing_bag_tokens = containing_bag_desc_str.split(" ");
                let count = containing_bag_tokens.next().unwrap()
                    .parse::<i64>().unwrap();
                let containing_bag = format!("{} {}",
                    containing_bag_tokens.next().unwrap(), //adjective
                    containing_bag_tokens.next().unwrap()  //color
                );

                let consuming_bags = baggage_map.entry(containing_bag.to_string())
                    .or_insert(HashSet::new());
                consuming_bags.insert(consuming_bag.to_string());

                baggage_counts.insert(containing_bag.to_string(), count);
            }
        }
        containment_map.insert(consuming_bag.to_string(), baggage_counts);
    }

    return (baggage_map, containment_map);
}

fn count_consuming_bags(
    start_bag: String,
    baggage_map: &HashMap<String, HashSet<String>>
) -> usize {
    let mut visited = HashSet::new();
    let mut queue = VecDeque::new();
    queue.push_back(start_bag.to_string());
    visited.insert(start_bag.to_string());
    
    while let Some(next_bag) = queue.pop_front() {
        match baggage_map.get(&next_bag) {
            Some(more_bags_to_try) => {
                for bag in more_bags_to_try {
                    if !visited.contains(bag) {
                        visited.insert(bag.to_string());
                        queue.push_back(bag.to_string());
                    }
                }
            },
            None => ()
        }

    }

    return visited.len() - 1;
}

/*
fn count_bags_inside_impl(
    current_bag: String,
    containment_map: &HashMap<String, HashMap<String, i64>>,
    already_computed: &mut HashMap<String, i64>
) -> i64 {
    let entry = already_computed.entry(current_bag.to_string()).or_insert_with(|| {
        let containments = containment_map.get(&current_bag).unwrap();
        let mut containment_count = 1; // start at 1 to include the bag itself

        for (k, v) in containments {
            containment_count += v * count_bags_inside_impl(
                k.to_string(), containment_map, already_computed);
        }

        return containment_count;
    });
    return *entry;
}

fn count_bags_inside(
    start_bag: String,
    containment_map: &HashMap<String, HashMap<String, i64>>
) -> i64 {
    return count_bags_inside_impl(start_bag, containment_map, &mut HashMap::new());
}
*/

fn count_bags_inside(
    start_bag: String,
    containment_map: &HashMap<String, HashMap<String, i64>>
) -> i64 {
    let containments = containment_map.get(&start_bag).unwrap();
    let mut containment_count = 1; // start at 1 to include the bag itself

    for (k, v) in containments {
        containment_count += v * count_bags_inside(
            k.to_string(),
            containment_map
        );
    }

    return containment_count;
}

fn main() {
    let (baggage_map, containment_map) = parse_input(&mut std::io::stdin().lock().lines());
    let num_consuming_bags = count_consuming_bags("shiny gold".to_string(), &baggage_map);
    println!("{}", num_consuming_bags);
    let num_bags_inside = count_bags_inside("shiny gold".to_string(), &containment_map) - 1;
    println!("{}", num_bags_inside);
}
