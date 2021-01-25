extern crate pest;
#[macro_use]
extern crate pest_derive;

use pest::Parser;
use std::collections::VecDeque;
use std::io::{stdin, Read};

#[derive(Parser)]
#[grammar = "deal.pest"]
struct DealParser;

#[derive(Debug)]
struct Engine {
    deck_1: VecDeque<u32>,
    deck_2: VecDeque<u32>,
}

impl Engine {
    fn new(input: &str) -> Engine {
        let mut player_id = 0;
        let mut deck_1 = VecDeque::new();
        let mut deck_2 = VecDeque::new();

        let pairs = DealParser::parse(Rule::main, input).unwrap_or_else(|e| panic!("{}", e));

        for pair in pairs {
            let rule = pair.as_rule();
            let text = pair.clone().as_span().as_str().to_string();

            match rule {
                Rule::player_id => {
                    player_id = text.parse::<u8>().unwrap();
                }
                Rule::value => match player_id {
                    1 => deck_1.push_back(text.parse::<u32>().unwrap()),
                    2 => deck_2.push_back(text.parse::<u32>().unwrap()),
                    _ => {}
                },
                _ => {}
            }
        }

        Engine { deck_1, deck_2 }
    }

    fn play_game(&mut self) -> u32 {
        loop {
            if self.deck_1.is_empty() {
                return self
                    .deck_2
                    .iter()
                    .zip((1..=self.deck_2.len()).rev())
                    .map(|(multiplicand, multiplier)| {
                        println!("{} x {}", multiplicand, multiplier);
                        multiplicand * multiplier as u32
                    })
                    .sum();
            }
            if self.deck_2.is_empty() {
                return self
                    .deck_1
                    .iter()
                    .zip((1..=self.deck_1.len()).rev())
                    .map(|(multiplicand, multiplier)| {
                        println!("{} x {}", multiplicand, multiplier);
                        multiplicand * multiplier as u32
                    })
                    .sum();
            }

            let card_1 = self.deck_1.pop_front().unwrap();
            let card_2 = self.deck_2.pop_front().unwrap();

            if card_1 > card_2 {
                self.deck_1.push_back(card_1);
                self.deck_1.push_back(card_2);
            } else {
                self.deck_2.push_back(card_2);
                self.deck_2.push_back(card_1);
            }
        }
    }
}

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();

    let mut engine = Engine::new(&input);

    let score = engine.play_game();
    println!("Part 1: the winning player's score is {}", score);
}
