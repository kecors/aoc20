extern crate pest;
#[macro_use]
extern crate pest_derive;

use pest::Parser;
use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::io::{stdin, Read};

#[derive(Parser)]
#[grammar = "step.pest"]
struct StepParser;

#[derive(Debug)]
enum Direction {
    East,
    Southeast,
    Southwest,
    West,
    Northwest,
    Northeast,
}

#[derive(Debug)]
struct Location {
    directions: Vec<Direction>,
    address: (i16, i16),
}

impl Location {
    fn new(directions: Vec<Direction>) -> Location {
        let mut address = (0, 0);
        for direction in directions.iter() {
            match direction {
                Direction::East => {
                    address.0 += 1;
                    address.1 += 1;
                }
                Direction::Southeast => address.1 += 1,
                Direction::Southwest => address.0 -= 1,
                Direction::West => {
                    address.0 -= 1;
                    address.1 -= 1;
                }
                Direction::Northwest => address.1 -= 1,
                Direction::Northeast => address.0 += 1,
            }
        }

        Location {
            directions,
            address,
        }
    }
}

#[derive(Debug)]
struct Engine {
    locations: Vec<Location>,
    addresses: HashMap<(i16, i16), bool>,
}

impl Engine {
    fn new(input: &str) -> Engine {
        let mut locations = Vec::new();

        for line in input.lines() {
            let mut directions = Vec::new();

            let pairs = StepParser::parse(Rule::main, line).unwrap_or_else(|e| panic!("{}", e));

            for pair in pairs {
                let rule = pair.as_rule();

                match rule {
                    Rule::east => directions.push(Direction::East),
                    Rule::southeast => directions.push(Direction::Southeast),
                    Rule::southwest => directions.push(Direction::Southwest),
                    Rule::west => directions.push(Direction::West),
                    Rule::northwest => directions.push(Direction::Northwest),
                    Rule::northeast => directions.push(Direction::Northeast),
                    _ => panic!("unexpected rule {:?}", rule),
                }
            }

            locations.push(Location::new(directions));
        }

        let mut addresses = HashMap::new();

        for location in locations.iter() {
            match addresses.entry(location.address) {
                Entry::Vacant(vacant) => {
                    vacant.insert(true);
                }
                Entry::Occupied(mut occupied) => {
                    let black_flag = occupied.get_mut();
                    *black_flag = !*black_flag;
                }
            }
        }

        Engine {
            locations,
            addresses,
        }
    }
}

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();

    let engine = Engine::new(&input);

    let sum = engine
        .addresses
        .iter()
        .filter(|(_, &black_flag)| black_flag)
        .count();
    println!("Part 1: {} tiles are black", sum);
}
