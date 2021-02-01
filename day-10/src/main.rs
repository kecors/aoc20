use std::collections::HashMap;
use std::io::{stdin, Read};

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();

    let mut p1_joltages: Vec<i16> = input.lines().map(|x| x.parse::<i16>().unwrap()).collect();
    let &max = p1_joltages.iter().max().unwrap();
    let mut p2_joltages = p1_joltages.clone();

    // Part 1

    p1_joltages.push(0);
    p1_joltages.push(max + 3);
    p1_joltages.sort_unstable();

    let mut differences: HashMap<i16, i16> = HashMap::new();

    for index in 0..p1_joltages.len() - 1 {
        let difference = p1_joltages[index + 1] - p1_joltages[index];
        let x = differences.entry(difference).or_insert(0);
        *x += 1;
    }

    let diff_1 = differences.get(&1).unwrap();
    let diff_3 = differences.get(&3).unwrap();
    let product = diff_1 * diff_3;
    println!("Part 1: the product is {}", product);

    // Part 2

    let mut hm: HashMap<i16, u64> = HashMap::new();
    hm.insert(0, 1);

    p2_joltages.sort_unstable();

    for &joltage in p2_joltages.iter() {
        let mut quantities = 0;
        if let Some(quantity) = hm.get(&(joltage - 3)) {
            quantities += quantity;
        }
        if let Some(quantity) = hm.get(&(joltage - 2)) {
            quantities += quantity;
        }
        if let Some(quantity) = hm.get(&(joltage - 1)) {
            quantities += quantity;
        }
        hm.insert(joltage, quantities);
    }

    let ways = hm.get(&max).unwrap();
    println!("Part 2: there are {} distinct ways", ways);
}
