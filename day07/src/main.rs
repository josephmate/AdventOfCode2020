// .lines()
use std::io::prelude::*;
use std::collections::HashSet;
use std::iter::FromIterator;
use std::collections::HashMap;

#[derive(Debug)]
struct CustomsForm {
    num_people: usize,
    answers: HashMap<char, usize>
}

fn parse_input(
    lines : &mut dyn Iterator<Item = Result<String, std::io::Error>>
) -> Vec<CustomsForm>{
    let mut result = Vec::new();

    let mut in_progress = false;
    let mut num_people = 0;
    let mut answers_so_far = HashMap::new();

    for line in lines {
        let line = line.unwrap();
        if line.is_empty() {
            result.push(CustomsForm {
                num_people: num_people,
                answers: answers_so_far
            });
            answers_so_far = HashMap::new();
            in_progress = false;
            num_people = 0;
        } else {
            num_people += 1;
            in_progress = true;
            for c in line.chars() {
                let count = answers_so_far.entry(c).or_insert(0);
                *count += 1;
            }
        }
    }
    
    if in_progress {
        result.push(CustomsForm {
            num_people: num_people,
            answers: answers_so_far
        });
    }

    return result;
}

fn main() {
    
    let customs_forms = parse_input(&mut std::io::stdin().lock().lines());

    println!("{}", customs_forms.iter()
        .map(|form| form.answers.len())
        .sum::<usize>()
    );

    println!("{}", customs_forms.iter()
        .map(|form| {
            form.answers.iter()
                .map(|(_, count)| count)
                .filter(|count| **count == form.num_people)
                .count()
        })
        .sum::<usize>()
    );
}
