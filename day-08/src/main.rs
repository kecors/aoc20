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

    fn run_p1(&self) -> i32 {
        let mut accumulator = 0;
        let mut ip = 0;
        let mut visited = HashSet::new();

        loop {
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

        accumulator
    }
}

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();

    let lines: Vec<&str> = input.lines().collect();

    let state = State::new(&lines);

    println!("The accumulator contains {}", state.run_p1());
}
