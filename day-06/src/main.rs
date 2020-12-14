use std::collections::HashSet;
use std::io::{stdin, Read};

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();

    let (mut groups, last_group) =
        input
            .lines()
            .fold((Vec::new(), Vec::new()), |(mut acc, mut group), x| {
                if x.is_empty() {
                    acc.push(group);
                    (acc, Vec::new())
                } else {
                    group.push(x);
                    (acc, group)
                }
            });
    groups.push(last_group);

    let mut sum = 0;
    for group in groups.iter() {
        let mut answers: Vec<char> = group.iter().map(|g| g.chars()).flatten().collect();
        answers.sort_unstable();
        answers.dedup();
        sum += answers.len();
    }

    println!("Part 1: the sum of the counts is {}", sum);

    let mut sum = 0;
    for group in groups.iter() {
        let person_hss: Vec<HashSet<char>> = group
            .iter()
            .map(|x| {
                let hs = x.chars().collect();
                hs
            })
            .collect();

        let mut common_hs: HashSet<char> = ('a'..='z').collect();
        for person_hs in person_hss.iter() {
            common_hs.retain(|x| person_hs.contains(&x));
        }

        sum += common_hs.len();
    }

    println!("Part 2: the sum of the counts is {}", sum);
}
