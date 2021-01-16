extern crate pest;
#[macro_use]
extern crate pest_derive;

use pest::Parser;
use std::collections::{HashMap, VecDeque};
use std::io::{stdin, Read};

#[derive(Parser)]
#[grammar = "item.pest"]
struct ItemParser;

#[derive(Debug)]
struct Subprecept {
    sequence: VecDeque<u16>,
}

#[derive(Debug)]
enum Precept {
    SingleCharacter(char),
    Subprecepts(Vec<Subprecept>),
}

#[derive(Debug, Clone)]
struct Context {
    sequence: VecDeque<u16>,
    message_offset: usize,
}

#[derive(Debug)]
struct Engine {
    precepts: HashMap<u16, Precept>,
    messages: Vec<Vec<char>>,
}

impl Engine {
    fn new(input: &str) -> Engine {
        let mut precepts = HashMap::new();
        let mut messages = Vec::new();
        let mut precept_id = 0;

        let pairs = ItemParser::parse(Rule::main, input).unwrap_or_else(|e| panic!("{}", e));

        for pair in pairs {
            let rule = pair.as_rule();
            let text = pair.clone().as_span().as_str().to_string();

            match rule {
                Rule::precept_id => {
                    precept_id = text.parse::<u16>().unwrap();
                }
                Rule::single_character => {
                    precepts.insert(
                        precept_id,
                        Precept::SingleCharacter(text.chars().next().unwrap()),
                    );
                }
                Rule::subprecepts => {
                    let subprecepts: Vec<Subprecept> = text
                        .split('|')
                        .map(|x| {
                            let sequence: VecDeque<u16> = x
                                .trim()
                                .split(' ')
                                .map(|y| y.parse::<u16>().unwrap())
                                .collect();
                            Subprecept { sequence }
                        })
                        .collect();
                    precepts.insert(precept_id, Precept::Subprecepts(subprecepts));
                }
                Rule::message => {
                    messages.push(text.chars().collect());
                }
                _ => {}
            }
        }

        Engine { precepts, messages }
    }

    fn part_2_fixup(&mut self) {
        let mut sequence_8a = VecDeque::new();
        sequence_8a.push_back(42);
        let mut sequence_8b = VecDeque::new();
        sequence_8b.push_back(42);
        sequence_8b.push_back(8);
        let precept_8 = Precept::Subprecepts(
            vec![
                Subprecept { sequence: sequence_8a },
                Subprecept { sequence: sequence_8b },
            ]
        );
        //dbg!(&precept_8);
        self.precepts.insert(8, precept_8);

        let mut sequence_11a = VecDeque::new();
        sequence_11a.push_back(42);
        sequence_11a.push_back(31);
        let mut sequence_11b = VecDeque::new();
        sequence_11b.push_back(42);
        sequence_11b.push_back(11);
        sequence_11b.push_back(31);
        let precept_11 = Precept::Subprecepts(
            vec![
                Subprecept { sequence: sequence_11a },
                Subprecept { sequence: sequence_11b },
            ]
        );
        //dbg!(&precept_11);
        self.precepts.insert(11, precept_11);
    }

    fn verify(&self, message: &[char]) -> bool {
        let mut stack = Vec::new();

        let mut sequence = VecDeque::new();
        sequence.push_back(0);
        let message_offset = 0;
        stack.push(Context {
            sequence,
            message_offset,
        });

        while let Some(mut context) = stack.pop() {
            //dbg!(&context);
            if let Some(precept_id) = context.sequence.pop_front() {
                if let Some(precept) = self.precepts.get(&precept_id) {
                    match precept {
                        Precept::SingleCharacter(c) => {
                            if *c == message[context.message_offset] {
                                let sequence = context.sequence.clone();
                                let message_offset = context.message_offset + 1;
                                if message.len() == message_offset {
                                    if context.sequence.len() == 0 {
                                        return true;
                                    }
                                } else {
                                    stack.push(Context {
                                        sequence,
                                        message_offset,
                                    });
                                }
                            }
                        }
                        Precept::Subprecepts(subprecepts) => {
                            for Subprecept { sequence } in subprecepts.iter() {
                                let mut new_sequence = sequence.clone();
                                let mut old_sequence = context.sequence.clone();
                                new_sequence.append(&mut old_sequence);
                                stack.push(Context {
                                    sequence: new_sequence,
                                    message_offset: context.message_offset,
                                });
                            }
                        }
                    }
                }
            }
        }

        false
    }

    fn count_matches(&self) -> usize {
        self.messages
            .iter()
            .filter(|message| self.verify(&message))
//            .inspect(|message| {
//                let s: String = message.into_iter().collect();
//                println!("{:?}", s)
//            })
            .count()
    }
}

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();

    let mut engine = Engine::new(&input);

    let sum = engine.count_matches();
    println!("Part 1: {} messages match rule 0", sum);

    engine.part_2_fixup();
    let sum = engine.count_matches();
    println!("Part 2: {} messages match rule 0", sum);
}
