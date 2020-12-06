// .lines()
use std::io::prelude::*;
use std::vec::Vec;
use std::convert::TryInto;

#[derive(Debug)]
struct Password {
    lower_bound: i64,
    upper_bound: i64,
    character_restriction: char,
    password: String
}

// 1-3 b: cdefg
// 1-3
// b:
// cdefg
fn parse_password(line: String) -> Password {
    let mut tokens = line.split(" ");
    let bounds_token = tokens.next().unwrap();
    let mut bounds_tokens = bounds_token.split("-");
    let lower_bound = bounds_tokens.next().unwrap().parse().unwrap();
    let upper_bound = bounds_tokens.next().unwrap().parse().unwrap();
    let character_restriction = tokens.next().unwrap().chars().next().unwrap();
    let password = tokens.next().unwrap();

    return Password {
        lower_bound: lower_bound,
        upper_bound: upper_bound,
        character_restriction: character_restriction,
        password: password.to_string()
    };
}

fn is_valid_sled(password: &Password) -> bool {
    let target_characters_count: i64 =
        password.password.chars()
            .filter(|c| *c == password.character_restriction)
            .count().try_into().unwrap();
    return password.lower_bound <= target_characters_count
        && target_characters_count <= password.upper_bound;
}

fn is_valid_toboggan(password: &Password) -> bool {
    let target_characters_count: i64 =
        password.password.char_indices()
            .filter(|(index, c)| (*index + 1) == password.lower_bound.try_into().unwrap()
                                    || (*index + 1) == password.upper_bound.try_into().unwrap())
            .filter(|(index, c)| *c == password.character_restriction)
            .count().try_into().unwrap();
    return target_characters_count == 1;
}

fn main() {
    let passwords : Vec<Password> = 
        std::io::stdin().lock().lines()
            .map(|l| parse_password(l.unwrap()))
            .collect();

    let mut valid_passwords = 0;
    for password in &passwords {
        if is_valid_sled(password) {
            valid_passwords += 1;
        }
    }
    println!("{}", valid_passwords);

    valid_passwords = 0;
    for password in &passwords {
        if is_valid_toboggan(password) {
            valid_passwords += 1;
        }
    }
    println!("{}", valid_passwords);
}
