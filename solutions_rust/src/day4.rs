use aoc_runner_derive::{aoc, aoc_generator};
use regex::{RegexSet, Regex};
use lazy_static;

#[aoc_generator(day4)]
pub fn input_generator(input: &str) -> Vec<String> {
    input.split("\n\n").map(|l| l.to_owned()).collect()
}

#[aoc(day4, part1)]
pub fn part_1(input: &[String]) -> usize {
    lazy_static! {
        static ref RE: RegexSet = RegexSet::new(&[
            r"byr:[^\s]*",
            r"iyr:[^\s]*",
            r"eyr:[^\s]*",
            r"hgt:[^\s]*",
            r"hcl:[^\s]*",
            r"ecl:[^\s]*",
            r"pid:[^\s]*",
        ]).unwrap();
    }

    input.iter()
        .filter(|l| RE.matches(l).into_iter().count() == 7)
        .count()
}

#[aoc(day4, part2)]
pub fn part_2(input: &[String]) -> usize {
    lazy_static! {
        static ref RE: RegexSet = RegexSet::new(&[
            r"byr:(192\d|19[3-9]\d|2000|2001|2002)\b",
            r"iyr:(201\d|2020)\b",
            r"eyr:(202\d|2030)\b",
            r"hgt:(59in|6\din|7[0-6]in|1[5-8]\dcm|19[0-3]cm)\b",
            r"hcl:#([0-9a-f]{6})\b",
            r"ecl:(amb|blu|brn|gry|grn|hzl|oth)\b",
            r"pid:\d{9}\b",
        ]).unwrap();
    }

    input.iter()
        .filter(|l| RE.matches(l).into_iter().count() == 7)
        .count()
}
