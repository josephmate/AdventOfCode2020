// .lines()
#[allow(unused_imports)]
use std::io::prelude::*;
#[allow(unused_imports)]
use std::collections::HashSet;
#[allow(unused_imports)]
use std::collections::HashMap;
#[allow(unused_imports)]
use std::collections::VecDeque;
#[allow(unused_imports)]
use std::env;
#[allow(unused_imports)]
use std::cmp::Ordering;

enum Rule {
  // (a & b) | (c & d)
  Or(Vec<usize>, Vec<usize>),
  And(Vec<usize>),
  Character(char),
}

fn parse_input(
  lines: &mut dyn Iterator<Item = String>
) -> (HashMap<usize, Rule>, Vec<String>) {
  let mut rules = HashMap::new();

  let mut current_line = lines.next().unwrap();
  while !current_line.is_empty() {
    /*
    0: 4 1 5
    1: 2 3 | 3 2
    2: 4 4 | 5 5
    3: 4 5 | 5 4
    4: "a"
    5: "b"
    */
    let mut rule_rest_tokens = current_line.split(": ");
    let rule_id = rule_rest_tokens.next().unwrap().parse::<usize>().unwrap();
    let rest = rule_rest_tokens.next().unwrap();
    let mut rule = Rule::Character('a');
    if rest.contains("|") {
      let mut or_tokens = rest.split(" | ");
      let left_side = or_tokens.next().unwrap();
      let right_side = or_tokens.next().unwrap();
      rule = Rule::Or(
        left_side.split(" ").map(|value_token| value_token.parse::<usize>().unwrap()).collect(),
        right_side.split(" ").map(|value_token| value_token.parse::<usize>().unwrap()).collect()
      );
    } else if rest.contains("a") {
      rule = Rule::Character('a');
    } else if rest.contains("b") {
      rule = Rule::Character('b');
    } else {
      rule = Rule::And(rest.split(" ").map(|value_token| value_token.parse::<usize>().unwrap()).collect());
    }
    rules.insert(rule_id, rule);
    current_line = lines.next().unwrap();
  }

  let remaining = lines.collect();
  return (rules, remaining);
}

fn flatten_rules(
  rule_id: usize,
  rules: &HashMap<usize, Rule>
) -> Vec<String> {
  return Vec::new();
}

fn eval_rules(
  rules: &Vec<String>,
  line: &String
) -> bool {
  return false;
}

fn main() {
  // let args: Vec<String> = env::args().collect();
  
  let (rules, input) = parse_input(&mut std::io::stdin().lock().lines()
    .map(|line_result| line_result.unwrap())
  );
  let falttened_rules = flatten_rules(&rules);

  println!("{}", input.iter()
    .filter(|line| eval_rules(&falttened_rules, &line))
    .count()
  );
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_samples() {
      
    }

}