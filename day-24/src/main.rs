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

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Location {
    nesw: i16,
    senw: i16,
}

impl Location {
    fn new(directions: Vec<Direction>) -> Location {
        let mut nesw: i16 = 0;
        let mut senw: i16 = 0;

        for direction in directions.iter() {
            match direction {
                Direction::East => {
                    senw += 1;
                    nesw += 1;
                }
                Direction::Southeast => senw += 1,
                Direction::Southwest => nesw -= 1,
                Direction::West => {
                    nesw -= 1;
                    senw -= 1;
                }
                Direction::Northwest => senw -= 1,
                Direction::Northeast => nesw += 1,
            }
        }

        Location { nesw, senw }
    }

    fn neighbors(&self) -> Vec<Location> {
        let mut neighbors = Vec::new();

        neighbors.push(Location {
            nesw: self.nesw + 1,
            senw: self.senw + 1,
        });
        neighbors.push(Location {
            nesw: self.nesw,
            senw: self.senw + 1,
        });
        neighbors.push(Location {
            nesw: self.nesw - 1,
            senw: self.senw,
        });
        neighbors.push(Location {
            nesw: self.nesw - 1,
            senw: self.senw - 1,
        });
        neighbors.push(Location {
            nesw: self.nesw,
            senw: self.senw - 1,
        });
        neighbors.push(Location {
            nesw: self.nesw + 1,
            senw: self.senw,
        });

        neighbors
    }
}

#[derive(Debug, PartialEq, Eq)]
enum Color {
    White,
    Black,
}

#[derive(Debug)]
struct Lobby {
    tiles: HashMap<Location, Color>,
}

impl Lobby {
    fn new() -> Lobby {
        Lobby {
            tiles: HashMap::new(),
        }
    }

    fn parse_input(&mut self, input: &str) {
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

            let location = Location::new(directions);
            self.flip(location);
        }
    }

    fn flip(&mut self, location: Location) {
        match self.tiles.entry(location) {
            Entry::Vacant(vacant) => {
                vacant.insert(Color::Black);
            }
            Entry::Occupied(mut occupied) => {
                let color = occupied.get_mut();
                match color {
                    Color::White => *color = Color::Black,
                    Color::Black => *color = Color::White,
                }
            }
        }
    }

    fn day(&mut self) {
        // Create a white tile for every neighboring location
        // of a black tile which does not have a tile
        let mut neighbors = Vec::new();
        for (location, _) in self.tiles.iter() {
            neighbors.append(&mut location.neighbors());
        }
        for neighbor in neighbors.into_iter() {
            match self.tiles.entry(neighbor) {
                Entry::Vacant(vacant) => {
                    vacant.insert(Color::White);
                }
                Entry::Occupied(_) => {}
            }
        }

        // Identify tiles which should be flipped
        let mut flip_tiles = Vec::new();
        for (location, color) in self.tiles.iter() {
            let mut black_count = 0;
            for n_location in location.neighbors() {
                if let Some(n_color) = self.tiles.get(&n_location) {
                    if *n_color == Color::Black {
                        black_count += 1;
                    }
                }
            }
            match color {
                Color::White => {
                    if black_count == 2 {
                        flip_tiles.push(location.clone());
                    }
                }
                Color::Black => {
                    if black_count == 0 || black_count > 2 {
                        flip_tiles.push(location.clone());
                    }
                }
            }
        }

        // Flip identified tiles
        for tile in flip_tiles {
            self.flip(tile);
        }
    }
}

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();

    let mut lobby = Lobby::new();
    lobby.parse_input(&input);

    let sum = lobby
        .tiles
        .iter()
        .filter(|&(_, color)| *color == Color::Black)
        .count();
    println!("Part 1: {} tiles are black", sum);

    for _ in 0..100 {
        lobby.day();
    }
    let sum = lobby
        .tiles
        .iter()
        .filter(|&(_, color)| *color == Color::Black)
        .count();
    println!("Part 2: {} tiles are black", sum);
}
