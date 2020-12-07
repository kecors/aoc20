use std::collections::HashSet;
use std::io::{stdin, Read};

fn proc_1(target_sum: u32, items: &[u32]) -> Option<(u32, u32)> {
    let mut values: HashSet<u32> = HashSet::new();

    for &item in items.iter() {
        if item < target_sum {
            let complement = target_sum - item;

            if values.contains(&complement) {
                return Some((item, complement));
            }
        }

        values.insert(item);
    }

    None
}

fn proc_2(mut items: Vec<u32>) -> Option<(u32, u32, u32)> {
    while let Some(entry_1) = items.pop() {
        if let Some((entry_2, entry_3)) = proc_1(2020 - entry_1, &items) {
            return Some((entry_1, entry_2, entry_3));
        }
    }

    None
}

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();

    let items: Vec<u32> = input
        .lines()
        .map(|x| x.trim().parse::<u32>().unwrap())
        .collect();

    if let Some((entry_1, entry_2)) = proc_1(2020, &items) {
        println!(
            "Part 1: the product of two entries ({}, {}) is {}",
            entry_1,
            entry_2,
            entry_1 * entry_2
        );
    } else {
        println!("Part 1 has no valid solution");
    }

    if let Some((entry_1, entry_2, entry_3)) = proc_2(items) {
        println!(
            "Part 2: the product of three entries ({}, {}, {}) is {}",
            entry_1,
            entry_2,
            entry_3,
            entry_1 * entry_2 * entry_3
        );
    } else {
        println!("Part 2 has no valid solution");
    }
}
