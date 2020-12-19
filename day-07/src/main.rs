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
struct Regulation {
    outer_bag_color: String,
    inner_bags: Vec<(u8, String)>,
}

#[derive(Debug)]
struct State {
    regulations: Vec<Regulation>,
    containables: HashMap<String, Vec<String>>,
}

impl State {
    fn new() -> State {
        let regulations = Vec::new();
        let containables = HashMap::new();

        State {
            regulations,
            containables,
        }
    }

    fn parse_line(&mut self, line: &str) {
        dbg!(&line);

        let pairs =
            RegulationParser::parse(Rule::regulation, line).unwrap_or_else(|e| panic!("{}", e));

        let mut outer_bag_color: String = String::new();
        let mut inner_bags: Vec<(u8, String)> = Vec::new();
        let mut inner_multiple_bags_quantity = 0;

        for pair in pairs {
            let rule = pair.as_rule();
            let text = pair.clone().as_span().as_str().to_string();

            match rule {
                Rule::outer_bag_color => {
                    outer_bag_color.push_str(&text);
                }
                Rule::inner_single_bag_color => {
                    inner_bags.push((1, text.clone()));
                    let vec = self
                        .containables
                        .entry(text.clone())
                        .or_insert_with(Vec::new);
                    vec.push(outer_bag_color.clone());
                }
                Rule::inner_multiple_bags_quantity => {
                    inner_multiple_bags_quantity = text.parse::<u8>().unwrap();
                }
                Rule::inner_multiple_bags_color => {
                    inner_bags.push((inner_multiple_bags_quantity, text.clone()));
                    let vec = self
                        .containables
                        .entry(text.clone())
                        .or_insert_with(Vec::new);
                    vec.push(outer_bag_color.clone());
                }
                _ => {
                    println!("unrecognized rule {}", &text);
                }
            }
        }

        self.regulations.push(Regulation {
            outer_bag_color,
            inner_bags,
        });
    }

    fn count_containable(self, target_color: &str) -> usize {
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
}

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();

    let lines: Vec<&str> = input.lines().collect();

    let mut state = State::new();

    for line in lines.iter() {
        state.parse_line(&line);
    }

    dbg!(&state);

    println!(
        "Part 1: {} bag colors can contain shiny gold",
        state.count_containable("shiny gold")
    );
}
