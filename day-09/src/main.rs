use std::cmp::Ordering;
use std::io::{stdin, Read};

static PREAMBLE_LENGTH: usize = 25;

fn find_invalid(numbers: &[u64]) -> Option<usize> {
    let mut candidate = PREAMBLE_LENGTH;

    while candidate < numbers.len() {
        if !is_valid(candidate, numbers) {
            return Some(candidate);
        }

        candidate += 1;
    }

    None
}

fn is_valid(target: usize, numbers: &[u64]) -> bool {
    let mut base = target - PREAMBLE_LENGTH;
    let mut other = base + 1;

    while base < target - 1 {
        while other < target {
            if numbers[base] + numbers[other] == numbers[target] {
                return true;
            }
            other += 1;
        }
        base += 1;
        other = base + 1;
    }

    false
}

fn find_weakness(invalid: usize, numbers: &[u64]) -> Option<u64> {
    let mut base = 0;
    let mut other = base + 1;

    while base < invalid - 1 {
        while other < invalid {
            let sum: u64 = numbers[base..=other].iter().sum();
            match sum.cmp(&numbers[invalid]) {
                Ordering::Equal => {
                    let min = numbers[base..=other].iter().min().unwrap();
                    let max = numbers[base..=other].iter().max().unwrap();
                    return Some(min + max);
                }
                Ordering::Greater => {
                    break;
                }
                Ordering::Less => {}
            }
            other += 1;
        }
        base += 1;
        other = base + 1;
    }

    None
}

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();

    let numbers: Vec<u64> = input.lines().map(|x| x.parse::<u64>().unwrap()).collect();

    if let Some(invalid) = find_invalid(&numbers) {
        println!("Part 1: the first invalid number is {}", numbers[invalid]);

        if let Some(weakness) = find_weakness(invalid, &numbers) {
            println!("Part 2: the encryption weakness is {}", weakness);
        }
    }
}
