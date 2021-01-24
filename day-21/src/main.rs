extern crate pest;
#[macro_use]
extern crate pest_derive;

use pest::Parser;
use std::collections::hash_map::Entry;
use std::collections::{HashMap, HashSet};
use std::io::{stdin, Read};

#[derive(Parser)]
#[grammar = "food.pest"]
struct FoodParser;

#[derive(Debug)]
struct Food {
    ingredients: Vec<String>,
    allergens: Vec<String>,
}

#[derive(Debug)]
struct Engine {
    foods: Vec<Food>,
}

impl Engine {
    fn new(input: &str) -> Engine {
        let mut foods = Vec::new();
        let mut ingredients = Vec::new();
        let mut allergens = Vec::new();

        let pairs = FoodParser::parse(Rule::main, input).unwrap_or_else(|e| panic!("{}", e));

        for pair in pairs {
            let rule = pair.as_rule();
            let text = pair.clone().as_span().as_str().to_string();

            match rule {
                Rule::ingredient => {
                    ingredients.push(text);
                }
                Rule::allergen => {
                    allergens.push(text);
                }
                Rule::food_end => {
                    foods.push(Food {
                        ingredients,
                        allergens,
                    });
                    ingredients = Vec::new();
                    allergens = Vec::new();
                }
                _ => {}
            }
        }

        Engine { foods }
    }

    fn count_appearances(&self) -> usize {
        let mut candidates: HashMap<String, HashSet<String>> = HashMap::new();

        // Identify list of ingredients which could contain each allergen
        for food in self.foods.iter() {
            let ingredients: HashSet<String> = food.ingredients.iter().cloned().collect();
            for allergen in food.allergens.iter() {
                match candidates.entry(allergen.clone()) {
                    Entry::Vacant(vacant) => {
                        vacant.insert(ingredients.clone());
                    }
                    Entry::Occupied(mut occupied) => {
                        let hs = occupied.get_mut();
                        *hs = hs.intersection(&ingredients).cloned().collect();
                    }
                }
            }
        }

        // Match each allergen with the ingredient that contains it
        let mut matches: HashMap<String, String> = HashMap::new();
        let mut questions_remain = true;
        while questions_remain {
            questions_remain = false;
            let mut new_candidates = HashMap::new();
            for (allergen, mut ingredients) in candidates.drain() {
                if ingredients.len() == 1 {
                    let ingredient = ingredients.drain().next().unwrap();
                    matches.insert(allergen, ingredient);
                } else {
                    questions_remain = true;
                    let matched: HashSet<String> = matches.values().cloned().into_iter().collect();
                    ingredients = ingredients.difference(&matched).cloned().collect();
                    new_candidates.insert(allergen, ingredients);
                }
            }
            candidates = new_candidates;
        }

        self.foods
            .iter()
            .map(|food| {
                food.ingredients
                    .iter()
                    .filter(|ingredient| !matches.values().any(|value| value == *ingredient))
                    .count()
            })
            .sum()
    }
}

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();

    let engine = Engine::new(&input);

    let appearances = engine.count_appearances();
    println!("Part 1: those ingredients appear {} times", appearances);
}
