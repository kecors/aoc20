use std::io::{stdin, Read};

#[derive(Debug, PartialEq, Clone)]
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

// Return the number of occupied seats among
// adjacent positions (including floor positions)
fn adjacent(layout: &[Vec<Position>], x: usize, y: usize) -> u8 {
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

// Return the number of occupied seats among
// visible seats (ignoring floor positions)
fn visible(layout: &[Vec<Position>], x: usize, y: usize) -> u8 {
    let mut count = 0;

    let deltas: Vec<(isize, isize)> = vec![
        (-1, -1),
        (0, -1),
        (1, -1),
        (-1, 0),
        (1, 0),
        (-1, 1),
        (0, 1),
        (1, 1),
    ];

    for delta in deltas.iter() {
        let mut cx: isize = x as isize;
        let mut cy: isize = y as isize;
        loop {
            cx += delta.0;
            cy += delta.1;
            if cx < 0 || cx >= layout[0].len() as isize || cy < 0 || cy >= layout.len() as isize {
                break;
            }
            match layout[cy as usize][cx as usize] {
                Position::Floor => {}
                Position::Empty => {
                    break;
                }
                Position::Occupied => {
                    count += 1;
                    break;
                }
            }
        }
    }

    count
}

fn apply_rules(
    old_layout: &[Vec<Position>],
    comparator: &dyn Fn(&[Vec<Position>], usize, usize) -> u8,
    threshold: u8,
) -> Vec<Vec<Position>> {
    let mut new_layout: Vec<Vec<Position>> = Vec::new();

    for y in 0..old_layout.len() {
        let mut line: Vec<Position> = Vec::new();

        for x in 0..old_layout[0].len() {
            match old_layout[y][x] {
                Position::Floor => {
                    line.push(Position::Floor);
                }
                Position::Empty => {
                    if comparator(&old_layout, x, y) == 0 {
                        line.push(Position::Occupied);
                    } else {
                        line.push(Position::Empty);
                    }
                }
                Position::Occupied => {
                    if comparator(&old_layout, x, y) >= threshold {
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

    let layout: Vec<Vec<Position>> = input
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

    // Part 1

    let mut p1_layout = layout.clone();
    //display_layout(&p1_layout);
    let mut old_occupied_seats = 0;
    let mut counter = 0;
    loop {
        p1_layout = apply_rules(&p1_layout, &adjacent, 4);
        //display_layout(&p1_layout);
        let occupied_seats = count_occupied_seats(&p1_layout);
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

    // Part 2

    let mut p2_layout = layout.clone();
    //display_layout(&p2_layout);
    let mut old_occupied_seats = 0;
    let mut counter = 0;
    loop {
        p2_layout = apply_rules(&p2_layout, &visible, 5);
        //display_layout(&p2_layout);
        let occupied_seats = count_occupied_seats(&p2_layout);
        if occupied_seats == old_occupied_seats {
            println!(
                "Part 2: {} seats end up occupied ({} applications)",
                occupied_seats, counter
            );
            break;
        }
        old_occupied_seats = occupied_seats;
        counter += 1;
    }
}
