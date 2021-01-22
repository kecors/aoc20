extern crate pest;
#[macro_use]
extern crate pest_derive;

use pest::Parser;
use std::collections::HashSet;
use std::io::{stdin, Read};

#[derive(Parser)]
#[grammar = "tile.pest"]
struct TileParser;

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
struct Image {
    rows: Vec<Vec<bool>>,
}

impl Image {
    fn new() -> Image {
        Image { rows: Vec::new() }
    }

    fn rotate(&self) -> Image {
        let mut rows = Vec::new();

        for x in 0..=self.rows.len() - 1 {
            let mut row = Vec::new();
            for y in (0..=self.rows.len() - 1).rev() {
                row.push(self.rows[y][x]);
            }
            rows.push(row);
        }

        Image { rows }
    }

    fn flip(&self) -> Image {
        let mut rows = Vec::new();

        for y in (0..=self.rows.len() - 1).rev() {
            let mut row = Vec::new();
            for x in 0..=self.rows.len() - 1 {
                row.push(self.rows[y][x]);
            }
            rows.push(row);
        }

        Image { rows }
    }

    fn generate_orientations(&self) -> Vec<Image> {
        let right = self.rotate();
        let bottom = right.rotate();
        let left = bottom.rotate();
        let top_flipped = self.flip();
        let right_flipped = right.flip();
        let bottom_flipped = bottom.flip();
        let left_flipped = left.flip();

        vec![
            self.clone(),
            right,
            bottom,
            left,
            top_flipped,
            right_flipped,
            bottom_flipped,
            left_flipped,
        ]
    }

    fn top(&self) -> Vec<bool> {
        let mut top = Vec::new();

        for x in (0..=self.rows.len() - 1).rev() {
            top.push(self.rows[0][x]);
        }

        top
    }

    fn right(&self) -> Vec<bool> {
        let mut right = Vec::new();

        for y in 0..=self.rows.len() - 1 {
            right.push(self.rows[y][self.rows.len() - 1]);
        }

        right
    }

    fn bottom(&self) -> Vec<bool> {
        let mut bottom = Vec::new();

        for x in 0..=self.rows.len() - 1 {
            bottom.push(self.rows[self.rows.len() - 1][x]);
        }

        bottom
    }

    fn left(&self) -> Vec<bool> {
        let mut left = Vec::new();

        for y in (0..=self.rows.len() - 1).rev() {
            left.push(self.rows[y][0]);
        }

        left
    }
}

#[derive(Debug, Clone)]
struct Tile {
    id: u32,
    image: Image,
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
                    for (j, row) in self.rows[sy][sx].image.rows.iter().enumerate() {
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
        let mut image = Image::new();

        let pairs = TileParser::parse(Rule::main, input).unwrap_or_else(|e| panic!("{}", e));

        for pair in pairs {
            let rule = pair.as_rule();
            let text = pair.clone().as_span().as_str().to_string();

            match rule {
                Rule::tile_id => {
                    if !image.rows.is_empty() {
                        parsed_tiles.push(Tile { id, image });
                        image = Image::new();
                    }
                    id = text.parse::<u32>().unwrap();
                }
                Rule::image_row => {
                    image.rows.push(text.chars().map(|x| x == '#').collect());
                }
                _ => {}
            }
        }
        parsed_tiles.push(Tile { id, image });
        let square_length = (parsed_tiles.len() as f64).sqrt() as usize;

        // Generate all rotations and flips for each tile
        let mut tiles = Vec::new();
        for tile in parsed_tiles.iter() {
            let mut new_tiles = tile
                .image
                .generate_orientations()
                .into_iter()
                .map(|image| Tile { id: tile.id, image })
                .collect();
            tiles.append(&mut new_tiles);
        }

        Engine {
            tiles,
            square_length,
        }
    }

    fn build_square(&self, cornerstone: Tile) -> Option<Square> {
        let mut used_tile_ids = HashSet::new();
        used_tile_ids.insert(cornerstone.id);

        let mut square = Square::new(cornerstone, self.square_length);

        // Fill the left column of the square
        for y in 0..self.square_length - 1 {
            if let Some(tile) = self
                .tiles
                .iter()
                .filter(|t| !used_tile_ids.contains(&t.id))
                .find(|t| is_matched(t.image.top(), square.rows[y][0].image.bottom()))
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
                    .filter(|t| {
                        y == 0 || !is_matched(t.image.top(), square.rows[y][x].image.bottom())
                    })
                    .find(|t| is_matched(t.image.left(), square.rows[y][x].image.right()))
                {
                    used_tile_ids.insert(tile.id);
                    square.rows[y].push(tile.clone());
                } else {
                    return None;
                }
            }
        }

        Some(square)
    }

    fn find_square(&self) -> Square {
        for tile in self.tiles.iter() {
            if let Some(square) = self.build_square(tile.clone()) {
                return square;
            }
        }

        panic!("No square found");
    }
}

#[derive(Debug)]
struct Seas {
    images: Vec<Image>,
}

impl Seas {
    fn new(square: &Square) -> Seas {
        let mut image = Image::new();

        for _ in 0..square.rows.len() {
            for _ in 1..=8 {
                image.rows.push(Vec::new());
            }
        }

        for sy in 0..square.rows.len() {
            for sx in 0..square.rows[0].len() {
                for (j, row) in square.rows[sy][sx].image.rows.iter().enumerate() {
                    if !(1..=8).contains(&j) {
                        continue;
                    }
                    for (k, pixel) in row.iter().enumerate() {
                        if !(1..=8).contains(&k) {
                            continue;
                        }
                        image.rows[sy * 8 + j - 1].push(*pixel);
                    }
                }
            }
        }

        let images = image.generate_orientations();

        Seas { images }
    }

    #[allow(dead_code)]
    fn display(&self) {
        for image in self.images.iter() {
            for row in image.rows.iter() {
                for pixel in row.iter() {
                    print!("{}", if *pixel { '#' } else { '.' });
                }
                println!();
            }
            println!();
        }
    }

    fn find_sea_monsters(&self) -> u16 {
        let monster = vec![
            0b00000000000000000010,
            0b10000110000110000111,
            0b01001001001001001000,
        ];
        let monster_bitcount = 15;
        let mut sea_bitcount = u16::MAX;

        for image in self.images.iter() {
            let mut image_bitcount: u16 = 0;
            let values: Vec<u128> = image
                .rows
                .iter()
                .map(|row| {
                    row.iter().rev().enumerate().fold(0, |mut acc, (x, b)| {
                        if *b {
                            image_bitcount += 1;
                            acc += 1 << x;
                        }
                        acc
                    })
                })
                .collect();
            for y in 0..values.len() - 2 {
                for x in 0..=image.rows.len() - 20 {
                    if values[y] >> x & monster[0] == monster[0]
                        && values[y + 1] >> x & monster[1] == monster[1]
                        && values[y + 2] >> x & monster[2] == monster[2]
                    {
                        image_bitcount -= monster_bitcount;
                    }
                }
            }
            sea_bitcount = sea_bitcount.min(image_bitcount);
        }

        sea_bitcount
    }
}

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();

    let engine = Engine::new(&input);

    let square = engine.find_square();
    let product = square.rows[0][0].id as u64
        * square.rows[0][square.length - 1].id as u64
        * square.rows[square.length - 1][0].id as u64
        * square.rows[square.length - 1][square.length - 1].id as u64;
    println!("Part 1: the product of the corner tile IDs is {}", product);

    let seas = Seas::new(&square);
    let roughness = seas.find_sea_monsters();
    println!("Part 2: the sea roughness is {}", roughness);
}
