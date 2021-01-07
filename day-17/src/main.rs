use std::cmp;
use std::collections::HashSet;
use std::io::{stdin, Read};

#[derive(Debug)]
struct Extent {
    min_x: i16,
    max_x: i16,
    min_y: i16,
    max_y: i16,
    min_z: i16,
    max_z: i16,
    min_w: i16,
    max_w: i16,
}

impl Extent {
    fn new(cubes: &HashSet<(i16, i16, i16, i16)>) -> Extent {
        let mut min_x = 0;
        let mut max_x = 0;
        let mut min_y = 0;
        let mut max_y = 0;
        let mut min_z = 0;
        let mut max_z = 0;
        let mut min_w = 0;
        let mut max_w = 0;

        for cube in cubes.iter() {
            min_x = cmp::min(min_x, cube.0);
            max_x = cmp::max(max_x, cube.0);
            min_y = cmp::min(min_y, cube.1);
            max_y = cmp::max(max_y, cube.1);
            min_z = cmp::min(min_z, cube.2);
            max_z = cmp::max(max_z, cube.2);
            min_w = cmp::min(min_w, cube.3);
            max_w = cmp::max(max_w, cube.3);
        }

        Extent {
            min_x,
            max_x,
            min_y,
            max_y,
            min_z,
            max_z,
            min_w,
            max_w,
        }
    }
}

#[derive(Debug)]
struct Engine {
    actives: HashSet<(i16, i16, i16, i16)>,
}

impl Engine {
    fn new(input: &str) -> Engine {
        let mut actives = HashSet::new();

        input.lines().rev().enumerate().for_each(|(y, line)| {
            line.chars()
                .enumerate()
                .filter(|(_, c)| *c == '#')
                .for_each(|(x, _)| {
                    actives.insert((x as i16, y as i16, 0, 0));
                })
        });

        Engine { actives }
    }

    fn count_active_neighbors(&self, cube: &(i16, i16, i16, i16)) -> usize {
        let mut neighbors = HashSet::new();

        for x in cube.0 - 1..=cube.0 + 1 {
            for y in cube.1 - 1..=cube.1 + 1 {
                for z in cube.2 - 1..=cube.2 + 1 {
                    for w in cube.3 - 1..=cube.3 + 1 {
                        neighbors.insert((x, y, z, w));
                    }
                }
            }
        }

        // A cube cannot be its own neighbor
        neighbors.remove(&(cube.0, cube.1, cube.2, cube.3));

        neighbors.intersection(&self.actives).count()
    }

    fn execute_cycle(&mut self, part: u8) {
        let mut new_actives = HashSet::new();

        let extent = Extent::new(&self.actives);

        for x in extent.min_x - 1..=extent.max_x + 1 {
            for y in extent.min_y - 1..=extent.max_y + 1 {
                for z in extent.min_z - 1..=extent.max_z + 1 {
                    for w in extent.min_w - 1..=extent.max_w + 1 {
                        if part == 1 && w != 0 {
                            continue;
                        }
                        let cube = (x, y, z, w);
                        match self.count_active_neighbors(&cube) {
                            2 => {
                                if self.actives.contains(&cube) {
                                    new_actives.insert(cube);
                                };
                            }
                            3 => {
                                new_actives.insert(cube);
                            }
                            _ => {}
                        }
                    }
                }
            }
        }

        self.actives = new_actives;
    }

    #[allow(dead_code)]
    fn display_cubes(&self) {
        let extent = Extent::new(&self.actives);

        for y in (extent.min_y..=extent.max_y).rev() {
            for x in extent.min_x..=extent.max_x {
                let cube = (x, y, 0, 0);
                if self.actives.contains(&cube) {
                    print!("#");
                } else {
                    print!(".");
                }
            }
            println!();
        }
        println!();
    }
}

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();

    let mut engine_p1 = Engine::new(&input);

    for _ in 0..6 {
        engine_p1.execute_cycle(1);
    }

    let count = engine_p1.actives.iter().count();
    println!("Part 1: {} cubes are active after six cycles", count);

    let mut engine_p2 = Engine::new(&input);

    for _ in 0..6 {
        engine_p2.execute_cycle(2);
    }

    let count = engine_p2.actives.iter().count();
    println!("Part 2: {} cubes are active after six cycles", count);
}
