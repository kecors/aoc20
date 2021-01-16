extern crate pest;
#[macro_use]
extern crate pest_derive;

use pest::Parser;
use std::collections::HashSet;
use std::io::{stdin, Read};

#[derive(Parser)]
#[grammar = "tile.pest"]
struct TileParser;

// Returns a copy of the pixelmap rotated to the right
fn rotate_pixelmap(pixelmap: &[Vec<bool>]) -> Vec<Vec<bool>> {
    let mut new_pixelmap = Vec::new();

    for x in 0..=9 {
        let mut row = Vec::new();
        for y in (0..=9).rev() {
            row.push(pixelmap[y][x]);
        }
        new_pixelmap.push(row);
    }

    new_pixelmap
}

// Returns a copy of the pixelmap flipped through the y axis
fn flip_pixelmap(pixelmap: &[Vec<bool>]) -> Vec<Vec<bool>> {
    let mut new_pixelmap = Vec::new();

    for y in (0..=9).rev() {
        let mut row = Vec::new();
        for x in 0..=9 {
            row.push(pixelmap[y][x]);
        }
        new_pixelmap.push(row);
    }

    new_pixelmap
}

fn is_matched(pixels_1: Vec<bool>, pixels_2: Vec<bool>) -> bool {
    if pixels_1.len() != pixels_2.len() {
        return false;
    }

    for (pixel_1, pixel_2) in pixels_1.iter().zip(pixels_2.iter().rev()) {
        if pixel_1 != pixel_2 {
            return false;
        }
    }

    true
}

#[derive(Debug, Clone)]
struct Tile {
    id: u32,
    pixelmap: Vec<Vec<bool>>,
}

impl Tile {
    fn top(&self) -> Vec<bool> {
        let mut top = Vec::new();
        for x in (0..=9).rev() {
            top.push(self.pixelmap[0][x]);
        }
        top
    }

    fn right(&self) -> Vec<bool> {
        let mut right = Vec::new();
        for y in 0..=9 {
            right.push(self.pixelmap[y][9]);
        }
        right
    }

    fn bottom(&self) -> Vec<bool> {
        let mut bottom = Vec::new();
        for x in 0..=9 {
            bottom.push(self.pixelmap[9][x]);
        }
        bottom
    }

    fn left(&self) -> Vec<bool> {
        let mut left = Vec::new();
        for y in (0..=9).rev() {
            left.push(self.pixelmap[y][0]);
        }
        left
    }
}

struct Square {
    rows: Vec<Vec<Tile>>,
    length: usize,
}

impl Square {
    fn new(cornerstone: Tile, square_length: usize) -> Square {
        Square {
            rows: vec![vec![cornerstone]],
            length: square_length,
        }
    }

    #[allow(dead_code)]
    fn display(&self) {
        let mut buffer: Vec<Vec<char>> = Vec::new();
        for _ in 0..self.length {
            for _ in 0..10 {
                buffer.push(Vec::new());
            }
        }

        for sy in 0..self.length {
            for sx in 0..self.length {
                if sy < self.length && sx < self.rows[sy].len() {
                    for (j, row) in self.rows[sy][sx].pixelmap.iter().enumerate() {
                        for pixel in row.iter() {
                            buffer[sy * 10 + j].push(if *pixel { '#' } else { '.' });
                        }
                    }
                } else {
                    for j in 0..10 {
                        buffer[sy * 10 + j].append(&mut vec!['-'; 10]);
                    }
                }
            }
        }

        for tile_row in self.rows.iter() {
            for tile in tile_row.iter() {
                print!("{} ", tile.id);
            }
            println!();
        }
        println!();

        for (y, row) in buffer.iter().enumerate() {
            if y > 0 && y % 10 == 0 {
                println!()
            }
            for (x, ch) in row.iter().enumerate() {
                if x > 0 && x % 10 == 0 {
                    print!(" ");
                }
                print!("{}", ch);
            }
            println!();
        }
        println!();
    }
}

#[derive(Debug)]
struct Engine {
    tiles: Vec<Tile>,
    square_length: usize,
}

impl Engine {
    fn new(input: &str) -> Engine {
        let mut parsed_tiles = Vec::new();
        let mut id = 0;
        let mut pixelmap: Vec<Vec<bool>> = Vec::new();

        let pairs = TileParser::parse(Rule::main, input).unwrap_or_else(|e| panic!("{}", e));

        for pair in pairs {
            let rule = pair.as_rule();
            let text = pair.clone().as_span().as_str().to_string();

            match rule {
                Rule::tile_id => {
                    if !pixelmap.is_empty() {
                        parsed_tiles.push(Tile { id, pixelmap });
                        pixelmap = Vec::new();
                    }
                    id = text.parse::<u32>().unwrap();
                }
                Rule::pixels_row => {
                    pixelmap.push(text.chars().map(|x| x == '#').collect());
                }
                _ => {}
            }
        }
        parsed_tiles.push(Tile { id, pixelmap });
        let square_length = (parsed_tiles.len() as f64).sqrt() as usize;

        // Generate all rotations and flips for each tile
        let mut tiles = Vec::new();
        for top in parsed_tiles.into_iter() {
            let right = Tile {
                id: top.id,
                pixelmap: rotate_pixelmap(&top.pixelmap),
            };
            let bottom = Tile {
                id: right.id,
                pixelmap: rotate_pixelmap(&right.pixelmap),
            };
            let left = Tile {
                id: bottom.id,
                pixelmap: rotate_pixelmap(&bottom.pixelmap),
            };

            let flipped_top = Tile {
                id: top.id,
                pixelmap: flip_pixelmap(&top.pixelmap),
            };
            let flipped_right = Tile {
                id: right.id,
                pixelmap: flip_pixelmap(&right.pixelmap),
            };
            let flipped_bottom = Tile {
                id: bottom.id,
                pixelmap: flip_pixelmap(&bottom.pixelmap),
            };
            let flipped_left = Tile {
                id: left.id,
                pixelmap: flip_pixelmap(&left.pixelmap),
            };

            tiles.push(top);
            tiles.push(right);
            tiles.push(bottom);
            tiles.push(left);
            tiles.push(flipped_top);
            tiles.push(flipped_right);
            tiles.push(flipped_bottom);
            tiles.push(flipped_left);
        }

        Engine {
            tiles,
            square_length,
        }
    }

    fn build_square(&self, cornerstone: Tile) -> Option<u64> {
        let mut used_tile_ids = HashSet::new();
        used_tile_ids.insert(cornerstone.id);

        let mut square = Square::new(cornerstone, self.square_length);

        // Fill the left column of the square
        for y in 0..self.square_length - 1 {
            if let Some(tile) = self
                .tiles
                .iter()
                .filter(|t| !used_tile_ids.contains(&t.id))
                .find(|t| is_matched(t.top(), square.rows[y][0].bottom()))
            {
                used_tile_ids.insert(tile.id);
                square.rows.push(vec![tile.clone()]);
            } else {
                return None;
            }
        }

        // Fill each row of the square
        for y in 0..self.square_length {
            for x in 0..self.square_length - 1 {
                if let Some(tile) = self
                    .tiles
                    .iter()
                    .filter(|t| !used_tile_ids.contains(&t.id))
                    .filter(|t| y == 0 || !is_matched(t.top(), square.rows[y][x].bottom()))
                    .find(|t| is_matched(t.left(), square.rows[y][x].right()))
                {
                    used_tile_ids.insert(tile.id);
                    square.rows[y].push(tile.clone());
                } else {
                    return None;
                }
            }
        }

        square.display();

        Some(
            square.rows[0][0].id as u64
                * square.rows[0][square.length - 1].id as u64
                * square.rows[square.length - 1][0].id as u64
                * square.rows[square.length - 1][square.length - 1].id as u64,
        )
    }

    fn solve(&self) -> Option<u64> {
        for tile in self.tiles.iter() {
            if let Some(product) = self.build_square(tile.clone()) {
                return Some(product);
            }
        }

        None
    }
}

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();

    let engine = Engine::new(&input);

    if let Some(product) = engine.solve() {
        println!("Part 1: the product of the corner tile IDs is {}", product);
    }
}
