use std::io::{stdin, Read};

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();

    let passes: Vec<&str> = input.lines().collect();
    dbg!(&passes);

    let seat_id_binary_strings: Vec<String> = passes
        .iter()
        .map(|x| {
            x.chars()
                .map(|y| match y {
                    'B' | 'R' => '1',
                    'F' | 'L' => '0',
                    _ => panic!("invalid input {}", y),
                })
                .collect()
        })
        .collect();
    dbg!(&seat_id_binary_strings);

    let seat_ids: Vec<u16> = seat_id_binary_strings
        .iter()
        .map(|x| u16::from_str_radix(x, 2).unwrap())
        .collect();
    dbg!(&seat_ids);

    let x = seat_ids.iter().max().unwrap();
    dbg!(&x);
}
