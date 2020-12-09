use std::io::{stdin, Read};

#[derive(Debug, PartialEq)]
enum Square {
    Tree,
    Open,
}

fn calculate_encounters(area: &[Vec<Square>], x_offset: usize, y_offset: usize) -> u32 {
    let mut x = 0;
    let mut y = 0;
    let mut tree_encounters = 0;

    while y < area.len() {
        if area[y][x % area[0].len()] == Square::Tree {
            tree_encounters += 1;
        }
        x += x_offset;
        y += y_offset;
    }

    tree_encounters
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

    println!(
        "Part 1: You would encounter {} trees",
        calculate_encounters(&area, 3, 1)
    );
    let tree_encounters = calculate_encounters(&area, 1, 1)
        * calculate_encounters(&area, 3, 1)
        * calculate_encounters(&area, 5, 1)
        * calculate_encounters(&area, 7, 1)
        * calculate_encounters(&area, 1, 2);
    println!("Part 2: You would encounter {} trees", tree_encounters);
}
