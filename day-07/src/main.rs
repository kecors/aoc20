extern crate pest;
#[macro_use]
extern crate pest_derive;

use pest::Parser;
use std::collections::{HashMap, HashSet};
use std::io::{stdin, Read};

#[derive(Parser)]
#[grammar = "regulation.pest"]
struct RegulationParser;

#[derive(Debug)]
struct State {
    containables: HashMap<String, Vec<String>>,
    containments: HashMap<String, Vec<(u32, String)>>,
}

impl State {
    fn new() -> State {
        let containables = HashMap::new();
        let containments = HashMap::new();

        State {
            containables,
            containments,
        }
    }

    fn parse_line(&mut self, line: &str) {
        let pairs =
            RegulationParser::parse(Rule::regulation, line).unwrap_or_else(|e| panic!("{}", e));

        let mut outer_bag_color: String = String::new();
        let mut inner_multiple_bags_quantity = 0;

        for pair in pairs {
            let rule = pair.as_rule();
            let text = pair.clone().as_span().as_str().to_string();

            match rule {
                Rule::outer_bag_color => {
                    outer_bag_color.push_str(&text);
                }
                Rule::inner_single_bag_color => {
                    let vec = self
                        .containables
                        .entry(text.clone())
                        .or_insert_with(Vec::new);
                    vec.push(outer_bag_color.clone());
                    let vec = self
                        .containments
                        .entry(outer_bag_color.clone())
                        .or_insert_with(Vec::new);
                    vec.push((1, text.clone()));
                }
                Rule::inner_multiple_bags_quantity => {
                    inner_multiple_bags_quantity = text.parse::<u32>().unwrap();
                }
                Rule::inner_multiple_bags_color => {
                    let vec = self
                        .containables
                        .entry(text.clone())
                        .or_insert_with(Vec::new);
                    vec.push(outer_bag_color.clone());
                    let vec = self
                        .containments
                        .entry(outer_bag_color.clone())
                        .or_insert_with(Vec::new);
                    vec.push((inner_multiple_bags_quantity, text.clone()));
                }
                _ => {
                    println!("unrecognized rule {}", &text);
                }
            }
        }
    }

    fn count_containable(&mut self, target_color: &str) -> usize {
        let mut candidates = vec![target_color];
        let mut containable_hs: HashSet<&str> = HashSet::new();

        while let Some(color) = candidates.pop() {
            if let Some(vec) = self.containables.get(color) {
                for containable in vec.iter() {
                    containable_hs.insert(&containable);
                    candidates.push(&containable);
                }
            }
        }

        containable_hs.len()
    }

    fn calculate_containment(&self, target_color: &str) -> u32 {
        let mut count = 1;

        if let Some(vec) = self.containments.get(target_color) {
            for (quantity, color) in vec.iter() {
                count += quantity * self.calculate_containment(color);
            }
        }

        count
    }
}

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();

    let lines: Vec<&str> = input.lines().collect();

    let mut state = State::new();

    for line in lines.iter() {
        state.parse_line(&line);
    }

    println!(
        "Part 1: {} bag colors can contain shiny gold",
        state.count_containable("shiny gold")
    );

    println!(
        "Part 2: {} bags are required inside one shiny gold bag",
        state.calculate_containment("shiny gold") - 1
    );
}
