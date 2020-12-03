use aoc_runner_derive::{aoc, aoc_generator};
use regex::Regex;

pub struct PasswordValidator {
    first: usize,
    second: usize,
    character: char,
    password: String,
}

#[aoc_generator(day2)]
pub fn input_generator(input: &str) -> Vec<PasswordValidator> {
    lazy_static! {
        static ref PATTERN: Regex = Regex::new(r"^(\d+)-(\d+) (\w): (\w+)$").unwrap();
    }

    input
        .lines()
        .map(|l| {
            let captures = PATTERN.captures(l).unwrap();

            PasswordValidator {
                first: captures[1].parse().unwrap(),
                second: captures[2].parse().unwrap(),
                character: captures[3].chars().nth(0).unwrap(),
                password: captures[4].into(),
            }
        })
        .collect()
}

#[aoc(day2, part1)]
pub fn verify_password_count(input: &[PasswordValidator]) -> usize {
    input
        .iter()
        .filter(|l| {
            let character_count = l
                .password
                .chars()
                .into_iter()
                .filter(|&c| c == l.character)
                .count();

            character_count >= l.first && character_count <= l.second
        })
        .count()
}

#[aoc(day2, part2)]
pub fn verify_password_positions(input: &[PasswordValidator]) -> usize {
    input
        .iter()
        .filter(|l| {
            (l.password.chars().nth(l.first - 1).unwrap() == l.character)
                ^ (l.password.chars().nth(l.second - 1).unwrap() == l.character)
        })
        .count()
}
