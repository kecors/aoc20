extern crate pest;
#[macro_use]
extern crate pest_derive;

use pest::Parser;
use std::collections::HashMap;
use std::io::{stdin, Read};

#[derive(Parser)]
#[grammar = "command.pest"]
struct CommandParser;

#[derive(Debug, Clone)]
enum Bit {
    Retain,
    Set0,
    Set1,
}

#[derive(Debug)]
enum Command {
    Bitmask(Vec<Bit>),
    Write(u64, u64),
}

#[derive(Debug)]
struct State {
    commands: Vec<Command>,
    bitmask: Vec<Bit>,
    values: HashMap<u64, u64>,
}

impl State {
    fn new() -> State {
        let commands = Vec::new();
        let bitmask = Vec::new();
        let values = HashMap::new();

        State {
            commands,
            bitmask,
            values,
        }
    }

    fn parse_line(&mut self, line: &str) {
        let pairs = CommandParser::parse(Rule::command, line).unwrap_or_else(|e| panic!("{}", e));

        let mut bits = Vec::new();
        let mut address = 0;

        for pair in pairs {
            let rule = pair.as_rule();
            let text = pair.clone().as_span().as_str().to_string();

            match rule {
                Rule::bit => {
                    bits.push(match text.as_str() {
                        "X" => Bit::Retain,
                        "0" => Bit::Set0,
                        "1" => Bit::Set1,
                        _ => panic!("unexpected bit"),
                    });
                }
                Rule::address => {
                    address = text.parse::<u64>().unwrap();
                }
                Rule::value => {
                    let value = text.parse::<u64>().unwrap();
                    self.commands.push(Command::Write(address, value));
                }
                _ => {}
            }
        }

        if !bits.is_empty() {
            self.commands.push(Command::Bitmask(bits));
        }
    }

    fn execute(&mut self) {
        for command in self.commands.iter() {
            match command {
                Command::Bitmask(bitmask) => {
                    self.bitmask = bitmask.to_vec();
                }
                Command::Write(address, value) => {
                    let mut result = 0;
                    for (index, bit) in self.bitmask.iter().rev().enumerate() {
                        match bit {
                            Bit::Retain => {
                                result |= (1 << index) & value;
                            }
                            Bit::Set0 => {}
                            Bit::Set1 => {
                                result |= 1 << index;
                            }
                        }
                    }
                    self.values.insert(*address, result);
                }
            }
        }
    }
}

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();

    let mut state = State::new();

    for line in input.lines() {
        state.parse_line(&line);
    }

    state.execute();

    let mut sum = 0;
    for (_, value) in state.values.iter() {
        sum += value;
    }
    println!("The sum of all values in memory is {}", sum);
}
