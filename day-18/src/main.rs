// I struggled with this one, and then implemented an approach
// I found on reddit. It is described here:
// https://github.com/mebeim/aoc/blob/master/2020/README.md#day-18---operation-order

use std::collections::VecDeque;
use std::io::{stdin, Read};

#[derive(Debug)]
struct Engine {
    lines: Vec<VecDeque<char>>,
}

impl Engine {
    fn new(input: &str) -> Engine {
        let mut lines = Vec::new();

        for line in input.lines() {
            let mut chars = VecDeque::new();

            for ch in line.chars().filter(|x| *x != ' ') {
                chars.push_back(ch);
            }

            lines.push(chars);
        }

        Engine { lines }
    }

    fn compute_line_p1(&mut self, index: usize) -> u64 {
        let mut acc = 0;
        let mut add = true;

        while let Some(ch) = self.lines[index].pop_front() {
            match ch {
                '+' => {
                    add = true;
                }
                '*' => {
                    add = false;
                }
                '(' => {
                    let value = self.compute_line_p1(index);
                    acc = if add { acc + value } else { acc * value };
                }
                ')' => {
                    break;
                }
                _ => {
                    let digit: u64 = ch.to_digit(10).unwrap().into();
                    acc = if add { acc + digit } else { acc * digit };
                }
            }
        }

        acc
    }

    fn compute_line_p2(&mut self, index: usize) -> u64 {
        let mut acc = 0;
        let mut mult = 1;

        while let Some(ch) = self.lines[index].pop_front() {
            match ch {
                '+' => {}
                '*' => {
                    mult = acc;
                    acc = 0;
                }
                '(' => {
                    acc += mult * self.compute_line_p2(index);
                }
                ')' => {
                    break;
                }
                _ => {
                    let digit: u64 = ch.to_digit(10).unwrap().into();
                    acc += digit * mult;
                }
            }
        }

        acc
    }

    fn run(&mut self, part1_flag: bool) -> u64 {
        let mut result = 0;

        for index in 0..self.lines.len() {
            if part1_flag {
                result += self.compute_line_p1(index);
            } else {
                result += self.compute_line_p2(index);
            }
        }

        result
    }
}

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();

    let mut engine = Engine::new(&input);
    let sum = engine.run(true);
    println!("Part 1: the sum of the results is {}", sum);

    let mut engine = Engine::new(&input);
    let sum = engine.run(false);
    println!("Part 2: the sum of the results is {}", sum);
}

#[cfg(test)]
mod tests {
    #[test]
    fn sample1_part1() {
        use crate::Engine;

        let input = "1 + 2 * 3 + 4 * 5 + 6";
        let mut engine = Engine::new(&input);
        let sum = engine.run(true);
        assert_eq!(sum, 71);
    }

    #[test]
    fn sample1_part2() {
        use crate::Engine;

        let input = "1 + 2 * 3 + 4 * 5 + 6";
        let mut engine = Engine::new(&input);
        let sum = engine.run(false);
        assert_eq!(sum, 231);
    }

    #[test]
    fn sample2_part1() {
        use crate::Engine;

        let input = "1 + (2 * 3) + (4 * (5 + 6))";
        let mut engine = Engine::new(&input);
        let sum = engine.run(true);
        assert_eq!(sum, 51);
    }

    #[test]
    fn sample2_part2() {
        use crate::Engine;

        let input = "1 + (2 * 3) + (4 * (5 + 6))";
        let mut engine = Engine::new(&input);
        let sum = engine.run(false);
        assert_eq!(sum, 51);
    }

    #[test]
    fn sample3_part() {
        use crate::Engine;

        let input = "2 * 3 + (4 * 5)";
        let mut engine = Engine::new(&input);
        let sum = engine.run(true);
        assert_eq!(sum, 26);
    }

    #[test]
    fn sample3_part2() {
        use crate::Engine;

        let input = "2 * 3 + (4 * 5)";
        let mut engine = Engine::new(&input);
        let sum = engine.run(false);
        assert_eq!(sum, 46);
    }

    #[test]
    fn sample4_part1() {
        use crate::Engine;

        let input = "5 + (8 * 3 + 9 + 3 * 4 * 3)";
        let mut engine = Engine::new(&input);
        let sum = engine.run(true);
        assert_eq!(sum, 437);
    }

    #[test]
    fn sample4_part2() {
        use crate::Engine;

        let input = "5 + (8 * 3 + 9 + 3 * 4 * 3)";
        let mut engine = Engine::new(&input);
        let sum = engine.run(false);
        assert_eq!(sum, 1445);
    }

    #[test]
    fn sample5_part1() {
        use crate::Engine;

        let input = "5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))";
        let mut engine = Engine::new(&input);
        let sum = engine.run(true);
        assert_eq!(sum, 12240);
    }

    #[test]
    fn sample5_part2() {
        use crate::Engine;

        let input = "5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))";
        let mut engine = Engine::new(&input);
        let sum = engine.run(false);
        assert_eq!(sum, 669060);
    }

    #[test]
    fn sample6_part1() {
        use crate::Engine;

        let input = "((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2";
        let mut engine = Engine::new(&input);
        let sum = engine.run(true);
        assert_eq!(sum, 13632);
    }

    #[test]
    fn sample6_part2() {
        use crate::Engine;

        let input = "((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2";
        let mut engine = Engine::new(&input);
        let sum = engine.run(false);
        assert_eq!(sum, 23340);
    }
}
