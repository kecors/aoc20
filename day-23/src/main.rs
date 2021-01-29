use std::collections::HashMap;
use std::io::{stdin, Read};

#[derive(Debug)]
struct Circle {
    cups: HashMap<u32, u32>,
    maximum: u32,
    cursor: u32,
}

impl Circle {
    fn new(cups_slice: &[u32]) -> Circle {
        let mut cups: HashMap<u32, u32> = HashMap::new();
        let mut maximum = 0;
        let mut cursor = 0;
        let mut first_opt: Option<u32> = None;
        let mut owner_opt: Option<u32> = None;

        for &cup in cups_slice.iter() {
            if let Some(owner) = owner_opt {
                cups.insert(owner, cup);
            } else {
                first_opt = Some(cup);
                cursor = cup;
            }
            owner_opt = Some(cup);
            if cup > maximum {
                maximum = cup;
            }
        }

        if let Some(owner) = owner_opt {
            if let Some(first) = first_opt {
                cups.insert(owner, first);
            }
        }

        Circle {
            cups,
            maximum,
            cursor,
        }
    }

    fn run(&mut self) {
        let first = *self.cups.get(&self.cursor).unwrap();
        let second = *self.cups.get(&first).unwrap();
        let third = *self.cups.get(&second).unwrap();
        let remainder = *self.cups.get(&third).unwrap();

        let mut destination = self.cursor;
        self.cups.insert(self.cursor, remainder);
        self.cursor = remainder;

        loop {
            if destination == 1 {
                destination = self.maximum;
            } else {
                destination -= 1;
            }
            if destination != first && destination != second && destination != third {
                break;
            }
        }

        let follower = *self.cups.get(&destination).unwrap();
        self.cups.insert(destination, first);
        self.cups.insert(third, follower);
    }

    fn solve_part_1(&mut self) -> String {
        for _ in 0..100 {
            self.run();
        }

        let mut result = String::new();
        let mut owner = 1;
        loop {
            if let Some(&cup) = self.cups.get(&owner) {
                if cup == 1 {
                    break;
                }
                result.push_str(&format!("{}", cup));
                owner = cup;
            }
        }

        result
    }

    fn solve_part_2(&mut self) -> u64 {
        for _ in 0..10_000_000 {
            self.run();
        }

        let first = *self.cups.get(&1).unwrap();
        let second = *self.cups.get(&first).unwrap();

        first as u64 * second as u64
    }
}

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();

    let mut cups: Vec<u32> = input
        .trim()
        .chars()
        .map(|x| x.to_digit(10).unwrap_or(0) as u32)
        .collect();

    let mut circle_p1 = Circle::new(&cups);
    println!(
        "Part 1: the labels after 1 now are {}",
        circle_p1.solve_part_1()
    );

    let mut additional_cups = (10..=1_000_000).collect();
    cups.append(&mut additional_cups);
    let mut circle_p2 = Circle::new(&cups);
    println!("Part 2: the product is {}", circle_p2.solve_part_2());
}
