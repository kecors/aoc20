use std::io::{stdin, Read};

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();

    let mut numbers: Vec<u32> = input
        .trim()
        .split(',')
        .map(|x| x.parse::<u32>().unwrap())
        .collect();

    while numbers.len() < 2020 {
        let mut target_option: Option<u32> = None;
        let mut new_number_option: Option<u32> = None;
        for (index, &number) in numbers.iter().rev().enumerate() {
            if let Some(target) = target_option {
                if number == target {
                    new_number_option = Some(index as u32);
                    break;
                }
            } else {
                target_option = Some(number);
            }
        }
        if let Some(new_number) = new_number_option {
            numbers.push(new_number);
        } else {
            numbers.push(0);
        }
    }
    println!("Part 1: the 2020th number is {}", numbers[2019]);
}
