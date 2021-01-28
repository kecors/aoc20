use std::collections::VecDeque;
use std::io::{stdin, Read};

fn run(mut circle: VecDeque<u8>) -> VecDeque<u8> {
    let cursor = circle.pop_front().unwrap();
    let mut remainder = circle.split_off(3);
    let mut destination = cursor;

    loop {
        destination = match destination {
            1 => 9,
            _ => destination - 1,
        };
        if remainder.contains(&destination) {
            break;
        }
    }

    let mut index = 0;
    loop {
        if let Some(cup) = remainder.get(index) {
            if *cup == destination {
                break;
            }
        }
        index += 1;
    }
    let mut resolution = remainder.split_off(index + 1);

    let mut new_circle = VecDeque::new();
    new_circle.append(&mut remainder);
    new_circle.append(&mut circle);
    new_circle.append(&mut resolution);
    new_circle.push_back(cursor);

    new_circle
}

fn circle_string(circle: &VecDeque<u8>) -> String {
    circle.iter().map(|x| format!("{}", x)).collect::<String>()
}

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();

    let mut circle: VecDeque<u8> = input
        .trim()
        .chars()
        .map(|x| x.to_digit(10).unwrap_or(0) as u8)
        .collect::<VecDeque<u8>>();

    for _ in 0..100 {
        circle = run(circle);
    }

    loop {
        if let Some(cup) = circle.pop_front() {
            if cup == 1 {
                break;
            } else {
                circle.push_back(cup);
            }
        }
    }
    println!(
        "Part 1: the labels after 1 now are {}",
        circle_string(&circle)
    );
}
