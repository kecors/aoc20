use regex::Regex;
use std::collections::HashMap;
use std::io::{stdin, Read};

fn validate_passport_part_1(passport: &HashMap<&str, &str>) -> bool {
    passport.contains_key("byr")
        && passport.contains_key("iyr")
        && passport.contains_key("eyr")
        && passport.contains_key("hgt")
        && passport.contains_key("hcl")
        && passport.contains_key("ecl")
        && passport.contains_key("pid")
}

fn validate_passport_part_2(passport: &HashMap<&str, &str>) -> bool {
    let mut byr_bool = false;
    if let Ok(byr) = passport["byr"].parse::<u16>() {
        if byr >= 1920 && byr <= 2002 {
            byr_bool = true;
        }
    }

    let mut iyr_bool = false;
    if let Ok(iyr) = passport["iyr"].parse::<u16>() {
        if iyr >= 2010 && iyr <= 2020 {
            iyr_bool = true;
        }
    }

    let mut eyr_bool = false;
    if let Ok(eyr) = passport["eyr"].parse::<u16>() {
        if eyr >= 2020 && eyr <= 2030 {
            eyr_bool = true;
        }
    }

    let mut hgt_bool = false;
    let rx = Regex::new(r"^([0-9]{3})cm$").unwrap();
    if let Some(cap_cm) = rx.captures(passport["hgt"]) {
        if let Ok(cms) = cap_cm[1].parse::<u16>() {
            if cms >= 150 && cms <= 193 {
                hgt_bool = true;
            }
        }
    } else {
        let rx = Regex::new(r"^([0-9]{2})in$").unwrap();
        if let Some(cap_in) = rx.captures(passport["hgt"]) {
            if let Ok(ins) = cap_in[1].parse::<u16>() {
                if ins >= 59 && ins <= 76 {
                    hgt_bool = true;
                }
            }
        }
    }

    let rx = Regex::new(r"^#[0-9|a-f]{6}$").unwrap();
    let hcl_bool = rx.is_match(passport["hcl"]);

    let ecl_bool = matches!(
        passport["ecl"],
        "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth"
    );

    let rx = Regex::new(r"^[0-9]{9}$").unwrap();
    let pid_bool = rx.is_match(passport["pid"]);

    byr_bool && iyr_bool && eyr_bool && hgt_bool && hcl_bool && ecl_bool && pid_bool
}

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();

    // Consolidate multiline passports

    let (mut passport_lines, last_passport_line) =
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
    passport_lines.push(last_passport_line);

    // Create passport hash maps

    let mut passports = Vec::new();

    for passport_line in passport_lines.iter() {
        let mut hm = HashMap::new();

        let fields: Vec<&str> = passport_line.split(' ').collect();

        for field in fields.iter() {
            let kv: Vec<&str> = field.split(':').collect();
            hm.insert(kv[0], kv[1]);
        }

        passports.push(hm);
    }

    // Validate passports

    let mut valid_counter_part_1 = 0;
    let mut valid_counter_part_2 = 0;

    for passport in passports.iter() {
        if validate_passport_part_1(&passport) {
            valid_counter_part_1 += 1;
            if validate_passport_part_2(&passport) {
                valid_counter_part_2 += 1;
            }
        }
    }

    println!("Part 1: there are {} valid passports", valid_counter_part_1);
    println!("Part 2: there are {} valid passports", valid_counter_part_2);
}
