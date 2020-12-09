use std::io::{stdin, Read};

#[derive(Debug, PartialEq)]
enum Square {
    Tree,
    Open,
}

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();

    let area: Vec<Vec<Square>> = input
        .lines()
        .map(|x| {
            x.chars()
                .map(|z| if z == '#' { Square::Tree } else { Square::Open })
                .collect()
        })
        .collect();

    let mut x = 0;
    let mut y = 0;
    let mut tree_encounters = 0;

    while y < area.len() {
        if area[y][x % area[0].len()] == Square::Tree {
            tree_encounters += 1;
        }
        x += 3;
        y += 1;
    }

    println!("Part 1: You would encounter {} trees", tree_encounters);
}
