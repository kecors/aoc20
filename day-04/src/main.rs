use std::collections::HashMap;
use std::io::{stdin, Read};

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();

    let (mut passports, last_passport) =
        input
            .lines()
            .fold((Vec::new(), String::new()), |(mut acc, mut pp), x| {
                if x.is_empty() {
                    acc.push(pp);
                    (acc, String::new())
                } else {
                    if !pp.is_empty() {
                        pp.push(' ')
                    };
                    pp.push_str(&x);
                    (acc, pp)
                }
            });
    passports.push(last_passport);

    let mut valid_counter = 0;

    for passport in passports.iter() {
        let fields: Vec<&str> = passport.split(' ').collect();
        let mut hm = HashMap::new();

        for field in fields.iter() {
            let key_and_value: Vec<&str> = field.split(':').collect();
            hm.insert(key_and_value[0], key_and_value[1]);
        }

        if hm.contains_key("byr")
            && hm.contains_key("iyr")
            && hm.contains_key("eyr")
            && hm.contains_key("hgt")
            && hm.contains_key("hcl")
            && hm.contains_key("ecl")
            && hm.contains_key("pid")
        {
            valid_counter += 1;
        }
    }

    println!("Part 1: there are {} valid passports", valid_counter);
}
