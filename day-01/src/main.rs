use std::collections::HashSet;
use std::io::{stdin, Read};

fn solve(items: &Vec<u32>) -> u32 {
    let mut values: HashSet<u32> = HashSet::new();

    for &item in items.iter() {
        let complement: u32 = 2020 - item;

        if values.contains(&complement) {
            return item * complement;
        }

        values.insert(item);
    }

    panic!("No solution available");
}

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();

    let items: Vec<u32> = input
        .lines()
        .map(|x| x.trim().parse::<u32>().unwrap())
        .collect();

    let x = solve(&items);

    println!("Part 1: the product is {}", &x);
}
