use aoc_runner_derive::aoc;
use std::collections::HashSet;

#[aoc(day6, part1)]
pub fn day6_part1(input: &str) -> usize {
    input
        .split("\n\n")
        .map(|l| {
            l.chars()
                .filter(|&c| char::is_alphanumeric(c))
                .collect::<HashSet<char>>()
        })
        .map(|s| s.len())
        .sum()
}

#[aoc(day6, part2)]
pub fn day6_part2(input: &str) -> usize {
    input
        .split("\n\n")
        .map(|g| {
            g.lines()
                .map(|l| {
                    l.chars()
                        .filter(|&c| char::is_alphanumeric(c))
                        .collect::<HashSet<char>>()
                })
                .fold(None, |acc, set| match acc {
                    None => Some(set),
                    Some(v) => Some(v.intersection(&set).map(|c| *c).collect()),
                })
                .unwrap()
                .len()
        })
        .sum()
}
