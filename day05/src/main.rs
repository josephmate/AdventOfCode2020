// .lines()
use std::io::prelude::*;
use std::vec::Vec;

 
// byr (Birth Year)
// iyr (Issue Year)
// eyr (Expiration Year)
// hgt (Height)
// hcl (Hair Color)
// ecl (Eye Color)
// pid (Passport ID)
// cid (Country ID)
struct Passport {
    byr: Option<String>,
    iyr: Option<String>,
    eyr: Option<String>,
    hgt: Option<String>,
    hcl: Option<String>,
    ecl: Option<String>,
    pid: Option<String>
}

fn parse_passport_batch() ->  Vec<Passport> {
    let mut passports = Vec::new();

    let mut in_middle = false;
    let mut byr = None;
    let mut iyr = None;
    let mut eyr = None;
    let mut hgt = None;
    let mut hcl = None;
    let mut ecl = None;
    let mut pid = None;

    for line in std::io::stdin().lock().lines() {
        let line = line.unwrap();
        if line.is_empty() {
            passports.push(Passport {
                byr: byr,
                iyr: iyr,
                eyr: eyr,
                hgt: hgt,
                hcl: hcl,
                ecl: ecl,
                pid: pid
            });

            in_middle = false;
            byr = None;
            iyr = None;
            eyr = None;
            hgt = None;
            hcl = None;
            ecl = None;
            pid = None;
        } else {
            in_middle = true;
            let mut passport_fields = line.split(" ");
            while let Some(passport_field) = passport_fields.next() {
                let mut tokens = passport_field.split(":");
                let field_type = tokens.next().unwrap();
                let field_value = tokens.next().unwrap();
                match field_type {
                    "byr" => byr = Some(field_value.to_string()),
                    "iyr" => iyr = Some(field_value.to_string()),
                    "eyr" => eyr = Some(field_value.to_string()),
                    "hgt" => hgt = Some(field_value.to_string()),
                    "hcl" => hcl = Some(field_value.to_string()),
                    "ecl" => ecl = Some(field_value.to_string()),
                    "pid" => pid = Some(field_value.to_string()),
                    _ => ()
                }
            }
        }
    }
    if in_middle {
        passports.push(Passport {
            byr: byr,
            iyr: iyr,
            eyr: eyr,
            hgt: hgt,
            hcl: hcl,
            ecl: ecl,
            pid: pid
        });
    }

    return passports;
}

fn are_fields_present(passport : &Passport) -> bool {
    return passport.byr.is_some()
        && passport.iyr.is_some()
        && passport.eyr.is_some()
        && passport.hgt.is_some()
        && passport.hcl.is_some()
        && passport.ecl.is_some()
        && passport.pid.is_some();
}

fn is_year_valid(
    year : &Option<String>,
    lower_bound: i64,
    upper_bound: i64
) -> bool {
    match year {
        Some(year) => {
            if year.len() == 4 {
                match year.parse::<i64>() {
                    Ok(year) => return year >= lower_bound && year <= upper_bound,
                    Err(_) => return false,
                }
            } else {
                return false;
            }
        },
        None => return false,
    }
}

// byr (Birth Year) - four digits; at least 1920 and at most 2002.
fn is_byr_valid(byr : &Option<String>) -> bool {
    return is_year_valid(byr, 1920, 2002);
}

// iyr (Issue Year) - four digits; at least 2010 and at most 2020.
fn is_iyr_valid(iyr : &Option<String>) -> bool {
    return is_year_valid(iyr, 2010, 2020);
}

// eyr (Expiration Year) - four digits; at least 2020 and at most 2030.
fn is_eyr_valid(eyr : &Option<String>) -> bool {
    return is_year_valid(eyr, 2020, 2030);
}

fn is_height_valid(
    hgt : &String,
    lower_bound: i64,
    upper_bound: i64
) -> bool {
    let hgt_num = hgt.get(0..hgt.len()-2).unwrap().parse::<i64>();
    match hgt_num {
        Ok(hgt_num) => return hgt_num >= lower_bound && hgt_num <= upper_bound,
        Err(_) => return false,
    }
}

// hgt (Height) - a number followed by either cm or in:
//    If cm, the number must be at least 150 and at most 193.
//     If in, the number must be at least 59 and at most 76.
fn is_hgt_valid(hgt : &Option<String>) -> bool {
    match hgt {
        Some(hgt) => {
            if hgt.ends_with("cm") {
                return is_height_valid(hgt, 150, 193);
            } else if hgt.ends_with("in") {
                return is_height_valid(hgt, 59, 76);
            } else {
                return false;
            }
        },
        None => return false,
    }
}

fn is_0_9_or_a_f(c : char) -> bool {
    match c {
        '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' | 'a' | 'b' | 'c' | 'd' | 'e' | 'f' => return true,
        _ => return false,
    }
}

// hcl (Hair Color) - a # followed by exactly six characters 0-9 or a-f.
fn is_hcl_valid(hcl : &Option<String>) -> bool {
    match hcl {
        Some(hcl) => {
            if hcl.starts_with("#") && hcl.len() == 7 {
                for c in hcl.get(1..hcl.len()).unwrap().chars() {
                    if !is_0_9_or_a_f(c) {
                        return false;
                    }
                }
                return true;
            } else {
                return false;
            }
        },
        None => return false,
    }
}

// ecl (Eye Color) - exactly one of: amb blu brn gry grn hzl oth.
fn is_ecl_valid(ecl : &Option<String>) -> bool {
    match ecl {
        Some(ecl) => {
            match ecl.as_ref() {
                "amb"  | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth"
                    => return true,
                _ => return false,
            }
        },
        None => return false,
    }
}

// pid (Passport ID) - a nine-digit number, including leading zeroes.
fn is_pid_valid(pid : &Option<String>) -> bool {
    match pid {
        Some(pid) => {
            if pid.len() == 9 {
                match pid.parse::<i64>() {
                    Ok(pid) => return true,
                    Err(_) => return false,
                }
            } else {
                return false;
            }
        },
        None => return false,
    }
}

fn is_valid_passport(passport : &Passport) -> bool {
    return is_byr_valid(&passport.byr)
        && is_iyr_valid(&passport.iyr)
        && is_eyr_valid(&passport.eyr)
        && is_hgt_valid(&passport.hgt)
        && is_hcl_valid(&passport.hcl)
        && is_ecl_valid(&passport.ecl)
        && is_pid_valid(&passport.pid)
}

// byr (Birth Year) - four digits; at least 1920 and at most 2002.
// iyr (Issue Year) - four digits; at least 2010 and at most 2020.
// eyr (Expiration Year) - four digits; at least 2020 and at most 2030.
//hgt (Height) - a number followed by either cm or in:
//    If cm, the number must be at least 150 and at most 193.
//    If in, the number must be at least 59 and at most 76.
// hcl (Hair Color) - a # followed by exactly six characters 0-9 or a-f.
// ecl (Eye Color) - exactly one of: amb blu brn gry grn hzl oth.
// pid (Passport ID) - a nine-digit number, including leading zeroes.

// ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
// byr:1937 iyr:2017 cid:147 hgt:183cm

// iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
// hcl:#cfa07d byr:1929

fn main() {
    let passports = parse_passport_batch();

    let num_have_all_fields = passports.iter()
        .filter(|passport| are_fields_present(*passport))
        .count();
    println!("{}", num_have_all_fields);

    let num_valid_passports = passports.iter()
        .filter(|passport| is_valid_passport(*passport))
        .count();
    println!("{}", num_valid_passports);

}
