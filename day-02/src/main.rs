use regex::Regex;
use std::collections::HashMap;
use std::io::{stdin, Read};

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();

    let items: Vec<&str> = input.lines().collect();

    let rx = Regex::new(r"^(\d+)-(\d+) ([a-z]): ([a-z]+)$").unwrap();
    let mut counter: u32 = 0;

    for item in items.iter() {
        if let Some(cap) = rx.captures(item) {
            let min: u8 = cap[1].parse().unwrap();
            let max: u8 = cap[2].parse().unwrap();
            let letter: char = cap[3].chars().next().unwrap();
            let password: Vec<char> = cap[4].chars().collect();

            let mut hm = HashMap::new();
            for ch in password.iter() {
                *hm.entry(ch).or_insert(0) += 1;
            }

            if let Some(&amount) = hm.get(&letter) {
                if min <= amount && amount <= max {
                    counter += 1;
                }
            }
        } else {
            println!("Invalid input: {}", item);
        }
    }

    println!("Part 1: {} passwords are valid", counter);
}
