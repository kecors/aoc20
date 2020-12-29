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
    X,
    Zero,
    One,
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
    values_p1: HashMap<u64, u64>,
    values_p2: HashMap<u64, u64>,
}

impl State {
    fn new() -> State {
        let commands = Vec::new();
        let bitmask = Vec::new();
        let values_p1 = HashMap::new();
        let values_p2 = HashMap::new();

        State {
            commands,
            bitmask,
            values_p1,
            values_p2,
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
                        "X" => Bit::X,
                        "0" => Bit::Zero,
                        "1" => Bit::One,
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

    fn execute_p1(&mut self) {
        for command in self.commands.iter() {
            match command {
                Command::Bitmask(bitmask) => {
                    self.bitmask = bitmask.to_vec();
                }
                Command::Write(in_address, in_value) => {
                    let mut value = 0;
                    for (index, bit) in self.bitmask.iter().rev().enumerate() {
                        match bit {
                            Bit::X => {
                                value |= (1 << index) & in_value;
                            }
                            Bit::Zero => {}
                            Bit::One => {
                                value |= 1 << index;
                            }
                        }
                    }
                    self.values_p1.insert(*in_address, value);
                }
            }
        }
    }

    fn execute_p2(&mut self) {
        for command in self.commands.iter() {
            match command {
                Command::Bitmask(bitmask) => {
                    self.bitmask = bitmask.to_vec();
                }
                Command::Write(in_address, in_value) => {
                    let mut addresses = vec![0];
                    for (index, bit) in self.bitmask.iter().rev().enumerate() {
                        match bit {
                            Bit::X => {
                                let mut new_addresses = Vec::new();
                                for address in addresses.iter() {
                                    let new_address = address | (1 << index);
                                    new_addresses.push(new_address);
                                }
                                addresses.append(&mut new_addresses);
                            }
                            Bit::Zero => {
                                for address in &mut addresses {
                                    *address |= (1 << index) & in_address;
                                }
                            }
                            Bit::One => {
                                for address in &mut addresses {
                                    *address |= 1 << index;
                                }
                            }
                        }
                    }
                    for address in addresses.iter() {
                        self.values_p2.insert(*address, *in_value);
                    }
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

    state.execute_p1();
    let mut sum = 0;
    for (_, value) in state.values_p1.iter() {
        sum += value;
    }
    println!("Part 1: The sum of all values in memory is {}", sum);

    state.execute_p2();
    let mut sum = 0;
    for (_, value) in state.values_p2.iter() {
        sum += value;
    }
    println!("Part 2: The sum of all values in memory is {}", sum);
}
