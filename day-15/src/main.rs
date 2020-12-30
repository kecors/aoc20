use std::collections::HashMap;
use std::io::{stdin, Read};

#[derive(Debug)]
struct State {
    numbers: Vec<u64>,
    latest: HashMap<u64, u64>,
}

impl State {
    fn new(numbers: &[u64]) -> State {
        let mut latest = HashMap::new();
        for (index, &number) in numbers.iter().enumerate() {
            latest.insert(number, index as u64);
        }

        State {
            numbers: numbers.to_vec(),
            latest,
        }
    }

    fn compute_naive(&mut self, nth: usize) -> u64 {
        while self.numbers.len() < nth {
            let mut target_option: Option<u64> = None;
            let mut new_number_option: Option<u64> = None;
            for (index, &number) in self.numbers.iter().rev().enumerate() {
                if let Some(target) = target_option {
                    if number == target {
                        new_number_option = Some(index as u64);
                        break;
                    }
                } else {
                    target_option = Some(number);
                }
            }
            if let Some(new_number) = new_number_option {
                self.numbers.push(new_number);
            } else {
                self.numbers.push(0);
            }
        }

        self.numbers[nth - 1]
    }

    fn compute_optimized(&mut self, nth: u64) -> u64 {
        let mut number: u64 = 0;

        for index in self.latest.len() as u64..nth - 1 {
            let next_number = if let Some(old_index) = self.latest.get(&number) {
                index - old_index
            } else {
                0
            };
            self.latest.insert(number, index);
            number = next_number;
        }

        number
    }
}

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();

    let starting_numbers: Vec<u64> = input
        .trim()
        .split(',')
        .map(|x| x.parse::<u64>().unwrap())
        .collect();

    let mut state_p1 = State::new(&starting_numbers);
    println!(
        "Part 1: the 2020th number is {}",
        state_p1.compute_naive(2020)
    );

    let mut state_p2 = State::new(&starting_numbers);
    println!(
        "Part 2: the 30 millionth number is {}",
        state_p2.compute_optimized(30_000_000)
    );
}
