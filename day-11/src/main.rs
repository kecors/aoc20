use std::io::{stdin, Read};

#[derive(Debug, PartialEq)]
enum Position {
    Floor,
    Empty,
    Occupied,
}

#[allow(dead_code)]
fn display_layout(layout: &[Vec<Position>]) {
    for y in 0..layout.len() {
        for x in 0..layout[0].len() {
            print!(
                "{}",
                match layout[y][x] {
                    Position::Floor => '.',
                    Position::Empty => 'L',
                    Position::Occupied => '#',
                }
            );
        }
        println!();
    }
    println!();
}

fn count_occupied_seats(layout: &[Vec<Position>]) -> u32 {
    let mut count = 0;

    for y in 0..layout.len() {
        for x in 0..layout[0].len() {
            if layout[y][x] == Position::Occupied {
                count += 1;
            }
        }
    }

    count
}

fn count_adjacent(layout: &[Vec<Position>], x: usize, y: usize) -> u8 {
    let mut count = 0;

    if y > 0 {
        if x > 0 && layout[y - 1][x - 1] == Position::Occupied {
            count += 1;
        }

        if layout[y - 1][x] == Position::Occupied {
            count += 1;
        }

        if x < layout[0].len() - 1 && layout[y - 1][x + 1] == Position::Occupied {
            count += 1;
        }
    }

    if x > 0 && layout[y][x - 1] == Position::Occupied {
        count += 1;
    }

    if x < layout[0].len() - 1 && layout[y][x + 1] == Position::Occupied {
        count += 1;
    }

    if y < layout.len() - 1 {
        if x > 0 && layout[y + 1][x - 1] == Position::Occupied {
            count += 1;
        }

        if layout[y + 1][x] == Position::Occupied {
            count += 1;
        }

        if x < layout[0].len() - 1 && layout[y + 1][x + 1] == Position::Occupied {
            count += 1;
        }
    }

    count
}

fn apply_rules(old_layout: &[Vec<Position>]) -> Vec<Vec<Position>> {
    let mut new_layout: Vec<Vec<Position>> = Vec::new();

    for y in 0..old_layout.len() {
        let mut line: Vec<Position> = Vec::new();

        for x in 0..old_layout[0].len() {
            match old_layout[y][x] {
                Position::Floor => {
                    line.push(Position::Floor);
                }
                Position::Empty => {
                    if count_adjacent(&old_layout, x, y) == 0 {
                        line.push(Position::Occupied);
                    } else {
                        line.push(Position::Empty);
                    }
                }
                Position::Occupied => {
                    if count_adjacent(&old_layout, x, y) >= 4 {
                        line.push(Position::Empty);
                    } else {
                        line.push(Position::Occupied);
                    }
                }
            }
        }

        new_layout.push(line);
    }

    new_layout
}

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();

    let mut layout: Vec<Vec<Position>> = input
        .lines()
        .map(|x| {
            x.chars()
                .map(|y| match y {
                    '.' => Position::Floor,
                    'L' => Position::Empty,
                    _ => panic!("unexpected position {}", y),
                })
                .collect()
        })
        .collect();
    //display_layout(&layout);

    let mut old_occupied_seats = 0;
    let mut counter = 0;
    loop {
        layout = apply_rules(&layout);
        //display_layout(&layout);
        let occupied_seats = count_occupied_seats(&layout);
        if occupied_seats == old_occupied_seats {
            println!(
                "Part 1: {} seats end up occupied ({} applications)",
                occupied_seats, counter
            );
            break;
        }
        old_occupied_seats = occupied_seats;
        counter += 1;
    }
}
