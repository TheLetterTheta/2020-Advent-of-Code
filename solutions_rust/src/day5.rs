use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day5)]
pub fn input_generator(input: &str) -> Vec<u16> {
    input
        .lines()
        .map(|l| {
            l.chars().fold(0, |acc, r| {
                if r == 'F' || r == 'L' {
                    acc << 1
                } else {
                    (acc << 1) + 1
                }
            })
        })
        .collect()
}

#[aoc(day5, part1)]
pub fn part_1(input: &[u16]) -> u16 {
    *input.into_iter().max().unwrap()
}

#[inline]
fn sum_from_to(low: u16, high: u16) -> u16 {
    (low..=high).sum()
}

#[aoc(day5, part2)]
pub fn part_2(input: &[u16]) -> u16 {
    sum_from_to(*input.iter().min().unwrap(), *input.iter().max().unwrap())
        - input.iter().sum::<u16>()
}
