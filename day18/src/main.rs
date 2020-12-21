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



/*
    1 + 2 * 3 + 4 * 5 + 6
    \ /   /   /   /   /
      \  /   /   /   /
        \/   /   /   /
          \ /   /   /
            \  /   /
            \/   /
              \ /

    1 + (2 * 3) + (4 * (5 + 6))
    \    \ /       \    \  /
      \    /         \    \/
      \  /           \   /
        \/             \ /
          \            /
            \        /
              \    /
*/

fn matching_paren(
  line: &str,
  first_paren: usize,
) -> usize {
  let mut count = first_paren+1;
  let mut left_paren_count = 1;
  for c in line[first_paren+1 .. line.len()].chars() {
    match c {
      '(' => {
        left_paren_count += 1;
      },
      ')' => {
        left_paren_count -= 1;
      },
      _ => (),
    }
    if left_paren_count == 0 {
      break;
    }
    count += 1;
  }
  return count;
}

fn solve_expression1(line: String) -> String {
  if line.contains("(") {
    let first_paren = line.find("(").unwrap();
    let ending_paren = matching_paren(&line, first_paren);
    let sub_expression_with_paren = line[first_paren ..= ending_paren].to_string();
    let sub_expression = line[(first_paren+1) ..= (ending_paren-1)].to_string();
    let sub_expression_soln = solve_expression1(sub_expression);
    return solve_expression1(line.replacen(&sub_expression_with_paren, &sub_expression_soln, 1));
  } else if line.contains(" + ") || line.contains(" * ") {
    let mut tokens = line.split(" ");
    // 11 + 22 * 33
    let first_value = tokens.next().unwrap();
    let operator = tokens.next().unwrap();
    let second_value = tokens.next().unwrap();
    let result = match operator {
      "+" => first_value.parse::<i64>().unwrap() + second_value.parse::<i64>().unwrap(),
      _ => first_value.parse::<i64>().unwrap() * second_value.parse::<i64>().unwrap(),
    };
    let result = result.to_string();
    return solve_expression1(
      line.replacen(
        &(first_value.to_owned() + " " + operator + " " + second_value),
        &result,
        1
      )
    );
  }
  return line;
}

fn solve_expression2(line: String) -> String {
  if line.contains("(") {
    let first_paren = line.find("(").unwrap();
    let ending_paren = matching_paren(&line, first_paren);
    let sub_expression_with_paren = line[first_paren ..= ending_paren].to_string();
    let sub_expression = line[(first_paren+1) ..= (ending_paren-1)].to_string();
    let sub_expression_soln = solve_expression2(sub_expression);
    return solve_expression2(line.replacen(&sub_expression_with_paren, &sub_expression_soln, 1));
  } else if line.contains(" + ") {
    let mut tokens = line.split(" ");
    // 11 + 22 * 33
    let mut first_value = tokens.next().unwrap();
    let mut operator = tokens.next().unwrap();
    while operator != "+" {
      first_value = tokens.next().unwrap();
      operator = tokens.next().unwrap();
    }
    let second_value = tokens.next().unwrap();
    let result = first_value.parse::<i64>().unwrap() + second_value.parse::<i64>().unwrap();
    let result = result.to_string();
    return solve_expression2(
      line.replacen(
        &(first_value.to_owned() + " " + operator + " " + second_value),
        &result,
        1
      )
    );
  } else if line.contains(" * ") {
    // no special logic needed because at this point we only have *
    let mut tokens = line.split(" ");
    // 11 + 22 * 33
    let first_value = tokens.next().unwrap();
    tokens.next().unwrap(); // not needed
    let second_value = tokens.next().unwrap();
    let result = first_value.parse::<i64>().unwrap() * second_value.parse::<i64>().unwrap();
    let result = result.to_string();
    return solve_expression2(
      line.replacen(
        &(first_value.to_owned() + " * " + second_value),
        &result,
        1
      )
    );
  }
  return line;
}

fn main() {
  // let args: Vec<String> = env::args().collect();
  let input: Vec<String> = std::io::stdin().lock().lines()
    .map(|line_result| line_result.unwrap())
    .collect();
  println!("{}", input.iter()
    .map(|line| solve_expression1(line.to_string()))
    .map(|solved_expression| solved_expression.parse::<i64>().unwrap())
    .sum::<i64>()
  );
  println!("{}", input.iter()
    .map(|line| solve_expression2(line.to_string()))
    .map(|solved_expression| solved_expression.parse::<i64>().unwrap())
    .sum::<i64>()
  );

}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_samples() {
      assert_eq!(solve_expression1("1 + 2 * 3 + 4 * 5 + 6".to_string()), "71".to_string());
      assert_eq!(solve_expression1("1 + (2 * 3) + (4 * (5 + 6))".to_string()), "51".to_string());
      assert_eq!(solve_expression1("2 * 3 + (4 * 5)".to_string()), "26".to_string());
      assert_eq!(solve_expression1("5 + (8 * 3 + 9 + 3 * 4 * 3)".to_string()), "437".to_string());
      assert_eq!(solve_expression1("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))".to_string()), "12240".to_string());
      assert_eq!(solve_expression1("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2".to_string()), "13632".to_string());
    }

    #[test]
    fn test_samples2() {
      // 1 + 2 = 3
      // 3 + 4 = 7
      // 5 + 6 = 11
      // 3 * 7 * 11
      // 231
      assert_eq!(solve_expression2("1 + 2 * 3 + 4 * 5 + 6".to_string()), "231".to_string());
      assert_eq!(solve_expression2("1 + (2 * 3) + (4 * (5 + 6))".to_string()), "51".to_string());
      assert_eq!(solve_expression2("2 * 3 + (4 * 5)".to_string()), "46".to_string());
      assert_eq!(solve_expression2("5 + (8 * 3 + 9 + 3 * 4 * 3)".to_string()), "1445".to_string());
      assert_eq!(solve_expression2("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))".to_string()), "669060".to_string());
      assert_eq!(solve_expression2("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2".to_string()), "23340".to_string());
    }

    #[test]
    fn test_only_number() {
      assert_eq!(solve_expression1("123".to_string()), "123".to_string());
    }

    #[test]
    fn test_only_number_and_paren() {
      assert_eq!(solve_expression1("(123)".to_string()), "123".to_string());
    }

    #[test]
    fn test_redundant_paren() {
      assert_eq!(solve_expression1("((123))".to_string()), "123".to_string());
    }

    #[test]
    fn test_only_two() {
      assert_eq!(solve_expression1("4 + 3".to_string()), "7".to_string());
      assert_eq!(solve_expression1("4 * 3".to_string()), "12".to_string());
    }

    #[test]
    fn test_only_three() {
      assert_eq!(solve_expression1("4 + 3 * 2".to_string()), "14".to_string());
      assert_eq!(solve_expression1("2 * 3 + 4".to_string()), "10".to_string());
    }

    #[test]
    fn test_big_one() {
      /*
      ((7 + 9 * 5 + 6 + 6 + 7) + 3 + (8 * 8 + 8 + 4 + 4 + 4) + 5 + 6 + 6) * 2 + 6 * (5 * 3 * 6 * (3 * 8 + 9 + 5 * 6) + 2) + ((7 * 3 + 9 + 5) * 3 * 7) + 8
        (7 + 9 * 5 + 6 + 6 + 7) + 3 + (8 * 8 + 8 + 4 + 4 + 4) + 5 + 6 + 6
            (7 + 9 * 5 + 6 + 6 + 7)
              80 + 6 + 6 + 7
              99
            (8 * 8 + 8 + 4 + 4 + 4)
            84
          438
        (5 * 3 * 6 * (3 * 8 + 9 + 5 * 6) + 2)
          (3 * 8 + 9 + 5 * 6)
        ((7 * 3 + 9 + 5) * 3 * 7)
      */
      assert_eq!(solve_expression1("(7 + 9 * 5 + 6 + 6 + 7)".to_string()), "99".to_string());
      assert_eq!(solve_expression1("(8 * 8 + 8 + 4 + 4 + 4)".to_string()), "84".to_string());
      assert_eq!(solve_expression1("(334 + 3 + 84 + 5 + 6 + 6)".to_string()), "438".to_string());
    }
    

}