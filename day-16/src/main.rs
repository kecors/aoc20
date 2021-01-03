extern crate pest;
#[macro_use]
extern crate pest_derive;

use pest::Parser;
use regex::Regex;
use std::collections::HashSet;
use std::io::{stdin, Read};

#[derive(Parser)]
#[grammar = "notes.pest"]
struct NotesParser;

#[derive(Debug)]
enum Position {
    Uninitialized,
    Candidates(HashSet<usize>),
    Determined(usize),
    Solved(usize),
}

// Rule for ticket fields
#[derive(Debug)]
struct Field {
    description: String,
    lower_1: u32,
    upper_1: u32,
    lower_2: u32,
    upper_2: u32,
    position: Position,
}

impl Field {
    fn is_value_in_a_range(&self, value: u32) -> bool {
        if self.lower_1 <= value && value <= self.upper_1 {
            return true;
        }
        if self.lower_2 <= value && value <= self.upper_2 {
            return true;
        }

        false
    }

    fn remove_candidate(&mut self, position_index: usize) {
        let mut determined_position = None;

        match &mut self.position {
            Position::Candidates(candidates) => {
                candidates.remove(&position_index);
                if candidates.len() == 1 {
                    let index = candidates.drain().next();
                    determined_position = index;
                }
            }
            _ => {
                panic!("remove_candidate() called unexpectedly");
            }
        }

        if let Some(index) = determined_position {
            self.position = Position::Determined(index);
        }
    }
}

#[derive(Debug)]
struct Ticket {
    values: Vec<u32>,
    invalid_value_index: Option<usize>,
}

#[derive(Debug)]
struct State {
    fields: Vec<Field>,
    your_ticket: Ticket,
    nearby_tickets: Vec<Ticket>,
}

impl State {
    fn new(input: &str) -> State {
        let mut field_description = String::new();
        let mut lower_1 = 0;
        let mut upper_1 = 0;
        let mut lower_2 = 0;
        let mut fields = Vec::new();
        let mut your_ticket_option = None;
        let mut nearby_tickets = Vec::new();

        let pairs = NotesParser::parse(Rule::main, input).unwrap_or_else(|e| panic!("{}", e));

        for pair in pairs {
            let rule = pair.as_rule();
            let text = pair.clone().as_span().as_str().to_string();
            match rule {
                Rule::field_description => {
                    field_description = text.clone();
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
                    fields.push(Field {
                        description: field_description.clone(),
                        lower_1,
                        upper_1,
                        lower_2,
                        upper_2,
                        position: Position::Uninitialized,
                    });
                }
                Rule::your_ticket => {
                    let values = text
                        .trim()
                        .split(',')
                        .map(|x| x.parse::<u32>().unwrap())
                        .collect();
                    your_ticket_option = Some(Ticket {
                        values,
                        invalid_value_index: None,
                    });
                }
                Rule::nearby_ticket => {
                    let values = text
                        .trim()
                        .split(',')
                        .map(|x| x.parse::<u32>().unwrap())
                        .collect();
                    nearby_tickets.push(Ticket {
                        values,
                        invalid_value_index: None,
                    });
                }
                _ => {}
            }
        }

        let your_ticket = your_ticket_option.unwrap();

        State {
            fields,
            your_ticket,
            nearby_tickets,
        }
    }

    fn mark_invalid_tickets(&mut self) {
        for mut ticket in &mut self.nearby_tickets {
            for (index, &value) in ticket.values.iter().enumerate() {
                let mut is_valid_value = false;

                for field in self.fields.iter() {
                    if field.is_value_in_a_range(value) {
                        is_valid_value = true;
                        break;
                    }
                }
                if !is_valid_value {
                    ticket.invalid_value_index = Some(index);
                }
            }
        }
    }

    fn calculate_error_rate(&self) -> u32 {
        let mut invalid_values = Vec::new();

        for ticket in self.nearby_tickets.iter() {
            if let Some(index) = ticket.invalid_value_index {
                invalid_values.push(ticket.values[index]);
            }
        }

        invalid_values.iter().sum()
    }

    fn determine_field_order(&mut self) {
        // Initialized field position candidates
        let mut candidates = HashSet::new();
        for index in 0..self.fields.len() {
            candidates.insert(index);
        }
        for mut field in &mut self.fields {
            field.position = Position::Candidates(candidates.clone());
        }

        // Remove position candidates based on tickets
        for ticket in self.nearby_tickets.iter() {
            if ticket.invalid_value_index.is_some() {
                continue;
            }

            for (position_index, &value) in ticket.values.iter().enumerate() {
                for field in self.fields.iter_mut() {
                    if !field.is_value_in_a_range(value) {
                        field.remove_candidate(position_index);
                    }
                }
            }
        }

        // Determine and resolve positions by process of elimination
        loop {
            let mut determined_tuples = Vec::new();
            let mut solved_count = 0;

            for (field_index, field) in self.fields.iter_mut().enumerate() {
                match field.position {
                    Position::Determined(position_index) => {
                        determined_tuples.push((field_index, position_index));
                    }
                    Position::Solved(_) => {
                        solved_count += 1;
                    }
                    _ => {}
                }
            }

            if solved_count == self.fields.len() {
                break;
            }

            for (field_index, position_index) in determined_tuples {
                for field in self.fields.iter_mut() {
                    if let Position::Candidates(_) = field.position {
                        field.remove_candidate(position_index);
                    }
                }
                self.fields[field_index].position = Position::Solved(position_index);
            }
        }
    }

    fn calculate_departure_product(&self) -> u64 {
        let mut product: u64 = 1;

        let rx = Regex::new(r"^departure.*$").unwrap();

        for field in self.fields.iter() {
            if rx.is_match(&field.description) {
                if let Position::Solved(index) = field.position {
                    product *= self.your_ticket.values[index] as u64;
                }
            }
        }

        product
    }
}

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();

    let mut state = State::new(&input);

    state.mark_invalid_tickets();

    let error_rate = state.calculate_error_rate();
    println!("Part 1: the ticket scanning error rate = {}", error_rate);

    state.determine_field_order();

    let departure_product = state.calculate_departure_product();
    println!(
        "Part 2: the product of the departure fields is {}",
        departure_product
    );
}
