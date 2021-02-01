use std::io::{stdin, Read};

fn get_loop_size(subject_number: u64, public_key: u64) -> usize {
    let mut value = 1;

    let mut loop_size = 0;
    loop {
        loop_size += 1;
        value = (value * subject_number) % 20201227;
        if value == public_key {
            break;
        }
    }

    loop_size
}

fn transform(subject_number: u64, loop_size: usize) -> u64 {
    let mut value = 1;

    for _ in 0..loop_size {
        value = (value * subject_number) % 20201227;
    }

    value
}

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();

    let mut lines = input.lines();
    let card_public_key: u64 = lines.next().unwrap().parse::<u64>().unwrap();
    let door_public_key: u64 = lines.next().unwrap().parse::<u64>().unwrap();

    let card_loop_size = get_loop_size(7, card_public_key);
    let door_loop_size = get_loop_size(7, door_public_key);

    let result_cd = transform(door_public_key, card_loop_size);
    let result_dc = transform(card_public_key, door_loop_size);
    if result_cd != result_dc {
        println!("Unequal results {} and {}", result_cd, result_dc);
    }
    println!("Part 1: the encryption key is {}", result_cd);
}
