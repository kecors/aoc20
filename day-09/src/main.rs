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

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();

    let numbers: Vec<u64> = input.lines().map(|x| x.parse::<u64>().unwrap()).collect();

    if let Some(invalid) = find_invalid(&numbers) {
        println!("Part 1: the first invalid number is {}", numbers[invalid]);
    }
}
