use std::collections::HashMap;
use std::io::{stdin, Read};

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();

    let mut joltages: Vec<u16> = input.lines().map(|x| x.parse::<u16>().unwrap()).collect();
    joltages.push(0);
    let &max = joltages.iter().max().unwrap();
    joltages.push(max + 3);
    joltages.sort_unstable();

    let mut differences: HashMap<u16, u16> = HashMap::new();

    for index in 0..joltages.len() - 1 {
        let difference = joltages[index + 1] - joltages[index];
        let x = differences.entry(difference).or_insert(0);
        *x += 1;
    }

    if let Some(diff_1) = differences.get(&1) {
        if let Some(diff_3) = differences.get(&3) {
            let product = *diff_1 * *diff_3;
            println!("Part 1: the product is {}", product);
        }
    }
}
