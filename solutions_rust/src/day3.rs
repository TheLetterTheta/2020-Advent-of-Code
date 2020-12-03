use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day3)]
pub fn input_generator(input: &str) -> Vec<Vec<bool>> {
    input
        .lines()
        .map(|l| l.chars().map(|c| c == '#').collect())
        .collect()
}

#[aoc(day3, part1)]
pub fn part_1(input: &[Vec<bool>]) -> usize {
    input
        .iter()
        .enumerate()
        .filter(|(k, l)| l[(3 * k) % l.len()])
        .count()
}

#[aoc(day3, part2)]
pub fn part_2(input: &[Vec<bool>]) -> usize {
    let sums = input
        .into_iter()
        .enumerate()
        .fold((0, 0, 0, 0, 0), |acc, (k, l)| {
            let len: usize = l.len();

            (
                acc.0 + l[k % len] as usize,
                acc.1 + l[(3 * k) % len] as usize,
                acc.2 + l[(5 * k) % len] as usize,
                acc.3 + l[(7 * k) % len] as usize,
                if k & 1 == 0 {
                    acc.4 + l[(k / 2) % len] as usize
                } else {
                    acc.4
                },
            )
        });

    sums.0 * sums.1 * sums.2 * sums.3 * sums.4
}
