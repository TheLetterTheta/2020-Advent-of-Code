use aoc_runner_derive::{aoc, aoc_generator};
use regex::Regex;

pub struct PasswordValidator {
    first: usize,
    second: usize,
    character: char,
    password: String
}

#[aoc_generator(day2)]
pub fn input_generator(input: &str) -> Vec<PasswordValidator> {

    let pattern = Regex::new(r"^(\d+)-(\d+) (\w): (\w+)$").unwrap();

    input.lines().map(|l| {
        let captures = pattern.captures(l).unwrap();

        PasswordValidator {
            first: captures.get(1).unwrap().as_str().parse().unwrap(),
            second: captures.get(2).unwrap().as_str().parse().unwrap(),
            character: captures.get(3).unwrap().as_str().chars().nth(0).unwrap(),
            password: captures.get(4).unwrap().as_str().into()
        }
    })
    .collect()
}

#[aoc(day2, part1)]
pub fn verify_password_count(input: &[PasswordValidator]) -> usize {
    input.iter().filter(|l| {
        let character_count = l.password.chars().into_iter().filter(|&c| c == l.character).count();
        
        character_count >= l.first && character_count <= l.second
    })
    .count()
}

#[aoc(day2, part2)]
pub fn verify_password_positions(input: &[PasswordValidator]) -> usize {
    input.iter().filter(|l| {
        (l.password.chars().nth(l.first - 1).unwrap() == l.character)
            ^ (l.password.chars().nth(l.second - 1).unwrap() == l.character)
    })
    .count()
}
