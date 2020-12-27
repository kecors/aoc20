use std::io::{stdin, Read};

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();

    let lines: Vec<&str> = input.lines().collect();
    let depart_time = lines[0].parse::<u32>().unwrap();
    let bus_ids: Vec<u32> = lines[1]
        .split(',')
        .filter(|x| *x != "x")
        .map(|x| x.parse::<u32>().unwrap())
        .collect();

    let delays: Vec<(u32, u32)> = bus_ids
        .iter()
        .map(|&x| ((x * ((depart_time / x) + 1) % depart_time), x))
        .collect();

    let choice = delays
        .iter()
        .min_by(|(a1, _), (a2, _)| a1.cmp(&a2))
        .unwrap();

    println!("Part 1: the product is {}", choice.0 * choice.1);
}
