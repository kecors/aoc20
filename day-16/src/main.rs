extern crate pest;
#[macro_use]
extern crate pest_derive;

use pest::Parser;
use std::io::{stdin, Read};

#[derive(Parser)]
#[grammar = "notes.pest"]
struct NotesParser;

// Rule for ticket fields
#[derive(Debug)]
struct Rftf {
    text: String,
    lower_1: u32,
    upper_1: u32,
    lower_2: u32,
    upper_2: u32,
}

#[derive(Debug)]
struct Ticket {
    values: Vec<u32>,
}

#[derive(Debug)]
struct State {
    rules: Vec<Rftf>,
    your_ticket: Ticket,
    nearby_tickets: Vec<Ticket>,
}

impl State {
    fn new(input: &str) -> State {
        let mut rules_text = String::new();
        let mut lower_1 = 0;
        let mut upper_1 = 0;
        let mut lower_2 = 0;
        let mut rules = Vec::new();
        let mut your_ticket_option = None;
        let mut nearby_tickets = Vec::new();

        let pairs = NotesParser::parse(Rule::main, input).unwrap_or_else(|e| panic!("{}", e));

        for pair in pairs {
            let rule = pair.as_rule();
            let text = pair.clone().as_span().as_str().to_string();
            match rule {
                Rule::rule_text => {
                    rules_text = text.clone();
                }
                Rule::lower_1 => {
                    lower_1 = text.parse::<u32>().unwrap();
                }
                Rule::upper_1 => {
                    upper_1 = text.parse::<u32>().unwrap();
                }
                Rule::lower_2 => {
                    lower_2 = text.parse::<u32>().unwrap();
                }
                Rule::upper_2 => {
                    let upper_2 = text.parse::<u32>().unwrap();
                    rules.push(Rftf {
                        text: rules_text.clone(),
                        lower_1,
                        upper_1,
                        lower_2,
                        upper_2,
                    });
                }
                Rule::your_ticket => {
                    let values = text
                        .trim()
                        .split(',')
                        .map(|x| x.parse::<u32>().unwrap())
                        .collect();
                    your_ticket_option = Some(Ticket { values });
                }
                Rule::nearby_ticket => {
                    let values = text
                        .trim()
                        .split(',')
                        .map(|x| x.parse::<u32>().unwrap())
                        .collect();
                    nearby_tickets.push(Ticket { values });
                }
                _ => {}
            }
        }

        let your_ticket = your_ticket_option.unwrap();

        State {
            rules,
            your_ticket,
            nearby_tickets,
        }
    }

    fn calculate_error_rate(&self) -> u32 {
        let mut invalid_values = Vec::new();

        for ticket in self.nearby_tickets.iter() {
            for &value in ticket.values.iter() {
                let mut is_rule_satisfied = false;

                for rule in self.rules.iter() {
                    if rule.lower_1 <= value && value <= rule.upper_1 {
                        is_rule_satisfied = true;
                        break;
                    }
                    if rule.lower_2 <= value && value <= rule.upper_2 {
                        is_rule_satisfied = true;
                        break;
                    }
                }
                if !is_rule_satisfied {
                    invalid_values.push(value);
                }
            }
        }

        invalid_values.iter().sum()
    }
}

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();

    let state = State::new(&input);

    let error_rate = state.calculate_error_rate();
    println!("Part 1: the ticket scanning error rate = {}", error_rate);
}
