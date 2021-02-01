use std::io::{stdin, Read};

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();

    let lines: Vec<&str> = input.lines().collect();
    let depart_time = lines[0].parse::<u64>().unwrap();

    // Part 1

    let bus_ids: Vec<u64> = lines[1]
        .split(',')
        .filter(|x| *x != "x")
        .map(|x| x.parse::<u64>().unwrap())
        .collect();

    let delays: Vec<(u64, u64)> = bus_ids
        .iter()
        .map(|&x| ((x * ((depart_time / x) + 1) % depart_time), x))
        .collect();

    let choice = delays
        .iter()
        .min_by(|(a1, _), (a2, _)| a1.cmp(&a2))
        .unwrap();

    println!("Part 1: the product is {}", choice.0 * choice.1);

    // Part 2

    // Use the large base only for the puzzle input, due to the hint
    //let base = 1;
    let base = 100_000_000_000;

    let (result, _) = lines[1]
        .split(',')
        .enumerate()
        .filter(|(_, bus_id)| *bus_id != "x")
        .map(|(offset, bus_id)| (offset, bus_id.parse::<u64>().unwrap()))
        .fold((base, 1), |(mut result, multiple), (offset, bus_id)| {
            //println!("({}, {}), ({}, {})", result, multiple, offset, bus_id);
            while (result + offset as u64) % bus_id != 0 {
                result += multiple;
            }
            (result, multiple * bus_id)
        });
    println!("Part 2: the earliest timestamp is {}", result);
}
