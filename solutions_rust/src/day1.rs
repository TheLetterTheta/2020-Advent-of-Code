use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::{HashSet, BTreeSet};
use std::ops::Bound::{Excluded};

#[aoc_generator(day1)]
pub fn input_generator(input: &str) -> Vec<u16> {
    input.lines().map(|l| l.parse::<u16>().unwrap() ).collect()
}

#[aoc(day1, part1)]
pub fn find_two(input: &[u16]) -> u32 {
    let mut needed = HashSet::new();

    for &num in input.iter() {
        let solution = 2020 - num;

        if needed.contains(&num) {
            return num as u32 * solution as u32;
        } else {
            needed.insert(solution);
        }
    }

    unreachable!();
}

#[aoc(day1, part2)]
pub fn find_three(input: &[u16]) -> u32 {
    let mut sorted_tree: BTreeSet<u16> = BTreeSet::new();

    for &first in input.iter() {
        let max: u16 = 2020 - first;
        
        let mut needed = HashSet::new();
        for &second in sorted_tree.range((Excluded(0), Excluded(max))) {
            let third = max - second;
            if needed.contains(&third) {
                return first as u32 * second as u32 * third as u32;
            } else {
                needed.insert(second);
            }
        }

        sorted_tree.insert(first);
    }

    unreachable!();
}
