use regex::Regex;
use std::io::{self, BufRead};

fn main() {
    // verify_passwords();
    part_2_verify_passwords();
}

fn part_2_verify_passwords() {
    let stdin = io::stdin();

    let pattern = Regex::new(r"^(\d+)-(\d+) (\w): (\w+)$").unwrap();
    let total = stdin.lock().lines().map(|l| l.unwrap()).filter(|l| {
        let capture_group = pattern.captures(l).unwrap();

        let first: usize = capture_group[1].parse().unwrap();
        let second: usize = capture_group[2].parse().unwrap();
        let character: char = capture_group[3].chars().nth(0).unwrap();
        let password = &capture_group[4];

        (password.chars().nth(first - 1).unwrap() == character)
            ^ (password.chars().nth(second - 1).unwrap() == character)

    }).count();

    eprintln!("Valid passwords: {}", total);
}


fn verify_passwords() {
    let stdin = io::stdin();

    let pattern = Regex::new(r"^(\d+)-(\d+) (\w): (\w+)$").unwrap();
    let total = stdin.lock().lines().map(|l| l.unwrap()).filter(|l| {
        let capture_group = pattern.captures(l).unwrap();

        let min: usize = capture_group[1].parse().unwrap();
        let max: usize = capture_group[2].parse().unwrap();
        let character: char = capture_group[3].chars().nth(0).unwrap();
        let password = &capture_group[4];

        let character_count = password.chars().into_iter().filter(|&c| c == character).count();

        character_count >= min && character_count <= max
    }).count();

    eprintln!("{}", total);
}
