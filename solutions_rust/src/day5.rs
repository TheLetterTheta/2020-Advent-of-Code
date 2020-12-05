use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day5)]
pub fn input_generator(input: &str) -> Vec<u32> {
    input
        .lines()
        .map(|l| {
            l.chars()
                .fold(0, |acc, r| {
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
pub fn part_1(input: &[u32]) -> u32 {
    *input
        .into_iter()
        .max()
        .unwrap()
}

#[inline]
fn sum_from_to (low: u32, high: u32) -> u32 {
    (low..=high).sum()
}

#[aoc(day5, part2)]
pub fn part_2(input: &[u32]) -> u32 {
    let (lower_bound, upper_bound, sum) = input
        .iter()
        .fold((None, None, 0), |acc, i| 
            match acc {
                (None, None, _) => (Some(i), Some(i), *i),
                (Some(low), Some(high), n) => {
                    if i < low {
                        (Some(i), Some(high), n + i)
                    } else if i > high {
                        (Some(low), Some(i), n + i)
                    } else {
                        (Some(low), Some(high), n + i)
                    }
                },
                _ => unreachable!()
            }
        );

    sum_from_to(*lower_bound.unwrap(),*upper_bound.unwrap()) - sum
}
