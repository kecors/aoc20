use regex::Regex;
use std::io::{stdin, Read};

#[derive(Debug)]
enum Facing {
    North,
    South,
    East,
    West,
}

#[derive(Debug)]
enum Action {
    North,
    South,
    East,
    West,
    Left,
    Right,
    Forward,
}

#[derive(Debug)]
struct Instruction {
    action: Action,
    value: i32,
}

#[derive(Debug)]
struct State {
    instructions: Vec<Instruction>,
}

impl State {
    fn new(lines: &[&str]) -> State {
        let mut instructions = Vec::new();

        let rx = Regex::new(r"^(.)(\d+)$").unwrap();

        for line in lines.iter() {
            if let Some(cap) = rx.captures(line) {
                let action = match &cap[1] {
                    "N" => Action::North,
                    "S" => Action::South,
                    "E" => Action::East,
                    "W" => Action::West,
                    "L" => Action::Left,
                    "R" => Action::Right,
                    "F" => Action::Forward,
                    _ => panic!("unknown operator {}", &cap[1]),
                };
                let value: i32 = cap[2].parse::<i32>().unwrap();
                instructions.push(Instruction { action, value });
            }
        }

        State { instructions }
    }

    fn execute_p1(&self) -> i32 {
        let mut facing = Facing::East;
        let mut x: i32 = 0;
        let mut y: i32 = 0;

        for instruction in self.instructions.iter() {
            match instruction.action {
                Action::North => {
                    y += instruction.value;
                }
                Action::South => {
                    y -= instruction.value;
                }
                Action::East => {
                    x += instruction.value;
                }
                Action::West => {
                    x -= instruction.value;
                }
                Action::Left => {
                    let mut degrees = 0;
                    while degrees < instruction.value {
                        facing = match facing {
                            Facing::North => Facing::West,
                            Facing::South => Facing::East,
                            Facing::East => Facing::North,
                            Facing::West => Facing::South,
                        };
                        degrees += 90;
                    }
                }
                Action::Right => {
                    let mut degrees = 0;
                    while degrees < instruction.value {
                        facing = match facing {
                            Facing::North => Facing::East,
                            Facing::South => Facing::West,
                            Facing::East => Facing::South,
                            Facing::West => Facing::North,
                        };
                        degrees += 90;
                    }
                }
                Action::Forward => match facing {
                    Facing::North => y += instruction.value,
                    Facing::South => y -= instruction.value,
                    Facing::East => x += instruction.value,
                    Facing::West => x -= instruction.value,
                },
            }
        }

        x.abs() + y.abs()
    }

    fn execute_p2(&self) -> i32 {
        let mut ship_x: i32 = 0;
        let mut ship_y: i32 = 0;
        let mut waypoint_x: i32 = 10;
        let mut waypoint_y: i32 = 1;

        for instruction in self.instructions.iter() {
            match instruction.action {
                Action::North => {
                    waypoint_y += instruction.value;
                }
                Action::South => {
                    waypoint_y -= instruction.value;
                }
                Action::East => {
                    waypoint_x += instruction.value;
                }
                Action::West => {
                    waypoint_x -= instruction.value;
                }
                Action::Left => {
                    let mut degrees = 0;
                    while degrees < instruction.value {
                        let old_x = waypoint_x;
                        let old_y = waypoint_y;
                        waypoint_x = -old_y;
                        waypoint_y = old_x;
                        degrees += 90;
                    }
                }
                Action::Right => {
                    let mut degrees = 0;
                    while degrees < instruction.value {
                        let old_x = waypoint_x;
                        let old_y = waypoint_y;
                        waypoint_x = old_y;
                        waypoint_y = -old_x;
                        degrees += 90;
                    }
                }
                Action::Forward => {
                    for _ in 0..instruction.value {
                        ship_x += waypoint_x;
                        ship_y += waypoint_y;
                    }
                }
            }
            println!(
                "{:?} {}: ship {},{}, waypoint {},{}",
                instruction.action, instruction.value, ship_x, ship_y, waypoint_x, waypoint_y
            );
        }

        ship_x.abs() + ship_y.abs()
    }
}

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();

    let lines: Vec<&str> = input.lines().collect();

    let state = State::new(&lines);

    println!("Part 1: the Manhattan distance is {}", state.execute_p1());
    println!("Part 2: the Manhattan distance is {}", state.execute_p2());
}
