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

fn parse_input(
  lines: &mut dyn Iterator<Item=String>
) -> Vec<(Vec<String>, Vec<String>)> {
  let mut result = Vec::new();

  for line in lines {
    // mxmxvkd kfcds sqjhc nhms
    // dairy, fish)
    let mut ingredients_and_alergens = line.split(" (contains ");
    let ingredients = ingredients_and_alergens.next().unwrap().split(' ')
      .map(|s| s.to_string())
      .collect::<Vec<String>>();
    // dairy, fish
    let alergens = ingredients_and_alergens.next().unwrap().split(')').next().unwrap().split(", ")
      .map(|s| s.to_string())
      .collect::<Vec<String>>();
    result.push((ingredients, alergens));
  }

  result
}

fn calc_allergan_to_potenial_ingredients(
  input: &[(Vec<String>, Vec<String>)]
) -> HashMap<String, HashSet<String>> {
  let mut allergen_to_ingredient_lists: HashMap<String,Vec<Vec<String>>> = HashMap::new();

  for (ingredients, allergens) in input {
    for allergen in allergens {
      let ingredient_lists = allergen_to_ingredient_lists.entry(allergen.to_string()).or_insert(Vec::new());
      (*ingredient_lists).push(ingredients.to_vec());
    }
  }

  let mut allergan_to_potenial_ingredients = HashMap::new();
  for (allergen, ingredient_lists) in allergen_to_ingredient_lists.iter() {
    let mut ingredient_lists = ingredient_lists.iter();
    let mut potential_ingredients = ingredient_lists.next().unwrap().iter()
      .map(|s| s.to_string())
      .collect::<HashSet<String>>();
    while let Some(ingredient_list) = ingredient_lists.next() {
      let ingredient_set = ingredient_list.iter()
        .map(|s| s.to_string())
        .collect::<HashSet<String>>();
      potential_ingredients.retain(|ingredient| ingredient_set.contains(ingredient));
    }
    allergan_to_potenial_ingredients.insert(allergen.to_string(), potential_ingredients);
  }

  allergan_to_potenial_ingredients
}

fn part1(
  input: &[(Vec<String>, Vec<String>)],
  allergan_to_potenial_ingredients: &HashMap<String, HashSet<String>>
) -> usize {
  

  let potential_ingredients = allergan_to_potenial_ingredients.values()
    .flatten()
    .map(|s| s.to_string())
    .collect::<HashSet<String>>();

  let all_ingredients = input.iter()
    .flat_map(|(ingredients, _allergens)| ingredients)
    .map(|s| s.to_string())
    .collect::<Vec<String>>();

  // Definately not allergan ingredients
  all_ingredients.iter()
    .filter(|ingredient| !potential_ingredients.contains(*ingredient))
    .count()
}

fn part2(
  mut allergan_to_potenial_ingredients: HashMap<String, HashSet<String>>
) -> Vec<String> {
  // sort all the allergens
  let mut sorted_allergens = allergan_to_potenial_ingredients.keys()
    .map(|s| s.to_string())
    .collect::<Vec<String>>();
  sorted_allergens.sort();

  let mut allergan_to_ingredient: HashMap<String, String> = HashMap::new();
  while !allergan_to_potenial_ingredients.is_empty() {
    let matched_allergen_allergen: Vec<(String, String)> = allergan_to_potenial_ingredients.iter()
      .filter(|(_, potential_ingredients)| potential_ingredients.len() == 1)
      .map(|(allergen, potential_ingredients)| (allergen.to_string(), potential_ingredients.iter().next().unwrap().to_string()))
      .collect();

      for (matched_allergen, _) in &matched_allergen_allergen {
        allergan_to_potenial_ingredients.remove(matched_allergen);
      }
      
      for (_, matched_ingredient) in &matched_allergen_allergen {
        for (_, potential_ingredients) in allergan_to_potenial_ingredients.iter_mut() {
          potential_ingredients.remove(matched_ingredient);
        }
      }

      for (matched_allergen, matched_ingredient) in &matched_allergen_allergen {
        allergan_to_ingredient.insert(matched_allergen.to_string(), matched_ingredient.to_string());
      }
  }

  // map allergens to their ingredients
  sorted_allergens.iter()
    .map(|allergen| allergan_to_ingredient[allergen].to_string())
    .collect()
}

fn main() {
  // let args: Vec<String> = env::args().collect();
  // &mut std::io::stdin().lock().lines()
  let input = parse_input(&mut std::io::stdin().lock().lines()
    .map(|line_result| line_result.unwrap()));

  let allergan_to_potenial_ingredients = calc_allergan_to_potenial_ingredients(&input);

  println!("{}", part1(&input, &allergan_to_potenial_ingredients));
  println!("{}", part2(allergan_to_potenial_ingredients).join(","));

}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_samples() {
      
    }

}