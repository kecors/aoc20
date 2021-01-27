extern crate pest;
#[macro_use]
extern crate pest_derive;

use pest::Parser;
use std::collections::{HashSet, VecDeque};
use std::io::{stdin, Read};

#[derive(Parser)]
#[grammar = "deal.pest"]
struct DealParser;

#[derive(Debug)]
enum Winner {
    Player1,
    Player2,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct State {
    deck_1: VecDeque<u32>,
    deck_2: VecDeque<u32>,
}

impl State {
    fn new(input: &str) -> State {
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

        State { deck_1, deck_2 }
    }
}

fn play_game(mut state: State, part2_flag: bool) -> (State, Winner) {
    let mut states = HashSet::new();

    loop {
        if state.deck_1.is_empty() {
            return (state, Winner::Player2);
        }
        if state.deck_2.is_empty() {
            return (state, Winner::Player1);
        }
        if !states.insert(state.clone()) {
            return (state, Winner::Player1);
        }

        let card_1 = state.deck_1.pop_front().unwrap();
        let card_2 = state.deck_2.pop_front().unwrap();

        let (_, winner) = if part2_flag
            && state.deck_1.len() as u32 >= card_1
            && state.deck_2.len() as u32 >= card_2
        {
            let deck_1 = state.deck_1.iter().cloned().take(card_1 as usize).collect();
            let deck_2 = state.deck_2.iter().cloned().take(card_2 as usize).collect();
            play_game(State { deck_1, deck_2 }, true)
        } else {
            (
                state.clone(),
                if card_1 > card_2 {
                    Winner::Player1
                } else {
                    Winner::Player2
                },
            )
        };
        match winner {
            Winner::Player1 => {
                state.deck_1.push_back(card_1);
                state.deck_1.push_back(card_2);
            }
            Winner::Player2 => {
                state.deck_2.push_back(card_2);
                state.deck_2.push_back(card_1);
            }
        }
    }
}

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();

    let state = State::new(&input);
    let (state, winner) = play_game(state, false);
    let deck = match winner {
        Winner::Player1 => state.deck_1,
        Winner::Player2 => state.deck_2,
    };
    let score: u32 = deck
        .iter()
        .zip((1..=deck.len()).rev())
        .map(|(multiplicand, multiplier)| {
            //println!("{} x {}", multiplicand, multiplier);
            multiplicand * multiplier as u32
        })
        .sum();
    println!("Part 1: the winning player's score is {}", score);

    let state = State::new(&input);
    let (state, winner) = play_game(state, true);
    let deck = match winner {
        Winner::Player1 => state.deck_1,
        Winner::Player2 => state.deck_2,
    };
    let score: u32 = deck
        .iter()
        .zip((1..=deck.len()).rev())
        .map(|(multiplicand, multiplier)| {
            //println!("{} x {}", multiplicand, multiplier);
            multiplicand * multiplier as u32
        })
        .sum();
    println!("Part 2: the winning player's score is {}", score);
}
