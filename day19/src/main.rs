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

#[derive(Debug)]
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

fn and_strings(
  a_strings: &HashSet<String>,
  b_strings: &HashSet<String>
) -> HashSet<String> {
  if a_strings.is_empty() {
    return b_strings.to_owned();
  }
  if b_strings.is_empty() {
    return a_strings.to_owned();
  }

  let mut result = HashSet::new();
  for a in a_strings {
    for b in b_strings {
      result.insert(a.to_string() + b);
    }
  }
  return result;
}

fn flatten_rules(
  rule_id: usize,
  rules: &HashMap<usize, Rule>
) -> HashSet<String> {
  match &rules[&rule_id] {
    Rule::Character(ch) => {
      return vec![ch.to_string()].into_iter().collect();
    },
    Rule::Or(a_rules, b_rules) => {
      let mut a_result = HashSet::new();
      for other in a_rules {
        a_result = and_strings(&a_result, &flatten_rules(*other, rules));
      }
      let mut b_result = HashSet::new();
      for other in b_rules {
        b_result = and_strings(&b_result, &flatten_rules(*other, rules));
      }
      return a_result.union(&b_result).map(|s| s.to_string()).collect();
    }
    Rule::And(and_rules) => {
      let mut and_result = HashSet::new();
      for other in and_rules {
        and_result = and_strings(&and_result, &flatten_rules(*other, rules));
      }
      return and_result;
    }
  }
}

fn flatten_rules_with_depth(
  rule_id: usize,
  depth: usize,
  rules: &HashMap<usize, Rule>
) -> Option<HashSet<String>> {
  if depth > 6 {
    return None;
  }
  match &rules[&rule_id] {
    Rule::Character(ch) => {
      return Some(vec![ch.to_string()].into_iter().collect());
    },
    Rule::Or(a_rules, b_rules) => {
      let mut a_result = HashSet::new();
      for other in a_rules {
        let opt_result = flatten_rules_with_depth(*other, depth + 1, rules);
        if opt_result.is_some() {
          a_result = and_strings(&a_result, &opt_result.unwrap());
        }
      }
      let mut b_result = HashSet::new();
      for other in b_rules {
        let opt_result = flatten_rules_with_depth(*other, depth + 1, rules);
        if opt_result.is_some() {
          b_result = and_strings(&b_result, &opt_result.unwrap());
        }
      }

      if a_result.is_empty() && b_result.is_empty() {
        return None;
      } else if a_result.is_empty() {
        return Some(b_result);
      } else if b_result.is_empty() {
        return Some(a_result);
      } else {
        return Some(a_result.union(&b_result).map(|s| s.to_string()).collect());
      }
    }
    Rule::And(and_rules) => {
      let mut and_result = HashSet::new();
      for other in and_rules {
        let opt_result = flatten_rules_with_depth(*other, depth + 1, rules);
        if opt_result.is_some() {
          and_result = and_strings(&and_result, &opt_result.unwrap());
        }
      }
      if and_result.is_empty() {
        return None;
      } else {
        return Some(and_result);
      }
    }
  }
}

fn eval_rules(
  rules: &HashSet<String>,
  line: &String
) -> bool {
  return rules.contains(line);
}

fn is_valid_substring(
  line: &str,
  rules: &HashMap<usize, Rule>
) -> bool {
  return false;
}

fn and(
  depth: usize,
  anded_rules: &Vec<usize>,
  line: &str,
  rules: &HashMap<usize, Rule>
) -> HashSet<usize> {
  let mut consumed = HashSet::new();
  consumed.insert(0);
  for rule in anded_rules {
    let mut new_consumed = HashSet::new();
    for current_idx in &consumed {
      let valid_paths = is_valid_impl(depth+1, *rule, &line[*current_idx .. line.len()], rules);
      let valid_paths: HashSet<usize> = valid_paths.iter()
        .map(|v| v+current_idx)
        .collect();
      //println!("{}and {} {:?} {:?} {} {}",
      //  std::iter::repeat(" ").take(depth*2).collect::<String>(),
      //  rule, valid_paths, consumed, (line.len() - current_idx), &line[*current_idx .. line.len()]);

      if !valid_paths.is_empty() {
        new_consumed = new_consumed.union(&valid_paths).map(|s| *s).collect();
      }
    }
    if new_consumed.is_empty() {
      return HashSet::new();
    }
    consumed = new_consumed;
  }
  return consumed.into_iter().collect();
}

fn is_valid_impl(
  depth: usize,
  current_rule_id: usize,
  line: &str,
  rules: &HashMap<usize, Rule>
      // whether or not the rules are satisfied
            // the number of characters consumed
) -> HashSet<usize> {
  if depth > 1000 {
    return HashSet::new();
  }
  if line.is_empty() {
    return HashSet::new();
  }
  let current_rule = &rules[&current_rule_id];
  //println!("{}{} {:?} {} {}",
  //  std::iter::repeat(" ").take(depth*2).collect::<String>(),
  //  current_rule_id, current_rule, line.len(), line);
  match current_rule {
    Rule::Character(ch) => {
      if line[0..1] == ch.to_string() {
        return vec![1].into_iter().collect();
      } else {
        return HashSet::new();
      }
    },
    Rule::Or(a_rules, b_rules) => {
      let a_valids = and(depth+1, a_rules, line, rules);
      let b_valids = and(depth+1, b_rules, line, rules);
      if !a_valids.is_empty() && !b_valids.is_empty() {
        return a_valids.union(&b_valids).map(|s| *s).collect();
      } else if !b_valids.is_empty() {
        return b_valids;
      } else if !a_valids.is_empty() {
        return a_valids;
      } else {
        return HashSet::new();
      }
    },
    Rule::And(and_rules) => {
      return and(depth+1, and_rules, line, rules);
    },
  }
}

fn is_valid(
  line: &str,
  rules: &HashMap<usize, Rule>
      // whether or not the rules are satisfied
            // the number of characters consumed
) -> bool {
  let valid_paths = is_valid_impl(0, 0, line, rules);
  return valid_paths.iter()
    .filter(|path_consumed| **path_consumed == line.len())
    .count() >= 1;
}

fn main() {
  // let args: Vec<String> = env::args().collect();
  
  let (mut rules, input) = parse_input(&mut std::io::stdin().lock().lines()
    .map(|line_result| line_result.unwrap())
  );
  let flattened_rules = flatten_rules(0, &rules);
  println!("flattened into {} rules", flattened_rules.len());
  println!("{}", input.iter()
    .filter(|line| eval_rules(&flattened_rules, &line))
    .count()
  );

  // part 2
  // As you look over the list of messages, you realize your matching rules aren't quite right.
  // To fix them, completely replace rules 8: 42 and 11: 42 31 with the following:
  // 8: 42 | 42 8
  // 11: 42 31 | 42 11 31
  rules.insert(8, Rule::Or(vec![42], vec![42, 8]));
  rules.insert(11, Rule::Or(vec![42, 31], vec![42, 11, 31]));
  println!("{}", input.iter()
    .filter(|line| is_valid(&line, &rules))
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