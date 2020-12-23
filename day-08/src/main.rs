use regex::Regex;
use std::collections::HashSet;
use std::io::{stdin, Read};

#[derive(Debug)]
enum Operator {
    Acc,
    Jmp,
    Nop,
}

#[derive(Debug)]
struct State {
    instructions: Vec<(Operator, i32)>,
}

impl State {
    fn new(lines: &[&str]) -> State {
        let mut instructions = Vec::new();

        let rx = Regex::new(r"^(acc|jmp|nop) ((\+|-)\d+)$").unwrap();
        for line in lines.iter() {
            if let Some(cap) = rx.captures(line) {
                let operator = match &cap[1] {
                    "acc" => Operator::Acc,
                    "jmp" => Operator::Jmp,
                    "nop" => Operator::Nop,
                    _ => panic!("unknown operator {}", &cap[1]),
                };
                let argument: i32 = cap[2].parse::<i32>().unwrap();
                instructions.push((operator, argument));
            }
        }

        State { instructions }
    }

    fn execute(&self) -> (bool, i32) {
        let mut accumulator = 0;
        let mut ip = 0;
        let mut visited = HashSet::new();

        loop {
            if ip >= self.instructions.len() {
                return (true, accumulator);
            }
            if visited.contains(&ip) {
                break;
            }
            visited.insert(ip);

            match self.instructions[ip].0 {
                Operator::Acc => {
                    accumulator += self.instructions[ip].1;
                    ip += 1;
                }
                Operator::Jmp => {
                    let offset = self.instructions[ip].1;
                    ip = (ip as i32 + offset) as usize;
                }
                Operator::Nop => {
                    ip += 1;
                }
            }
        }

        (false, accumulator)
    }

    fn run_p1(&self) -> i32 {
        let (_terminated, accumulator) = self.execute();

        accumulator
    }

    fn run_p2(&mut self) -> i32 {
        for index in 0..self.instructions.len() {
            match self.instructions[index].0 {
                Operator::Acc => {}
                Operator::Jmp => {
                    let argument = self.instructions[index].1;
                    self.instructions[index] = (Operator::Nop, argument);
                    let (terminated, accumulator) = self.execute();
                    if terminated {
                        return accumulator;
                    }
                    self.instructions[index] = (Operator::Jmp, argument);
                }
                Operator::Nop => {
                    let argument = self.instructions[index].1;
                    self.instructions[index] = (Operator::Jmp, argument);
                    let (terminated, accumulator) = self.execute();
                    if terminated {
                        return accumulator;
                    }
                    self.instructions[index] = (Operator::Nop, argument);
                }
            }
        }

        0
    }
}

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();

    let lines: Vec<&str> = input.lines().collect();

    let mut state = State::new(&lines);

    println!("Part 1: The accumulator contains {}", state.run_p1());
    println!("Part 2: The accumulator contains {}", state.run_p2());
}
