use aoc_runner_derive::{aoc, aoc_generator};
use regex::Regex;
use std::collections::{HashMap, HashSet};

#[aoc_generator(day7)]
pub fn input_generator(input: &str) -> HashMap<String, Vec<(usize, String)>> {

    lazy_static! {
        static ref PATTERN: Regex = Regex::new(r"^(?P<color>[\w ]+) bags contain (?P<contents>.+)$").unwrap();
        static ref CONTENTS: Regex = Regex::new(r"(?P<quantity>\d+) (?P<color>[\w ]+) bag(s)?").unwrap();
    }

    input.lines()
        .map(|l| {
            let parsed = PATTERN.captures(l).unwrap();
            let color = parsed.name("color").unwrap();
            let contents = parsed.name("contents").unwrap();

            let contents = contents
                .as_str()
                .split(", ")
                .filter_map(|c| {
                    let parsed = CONTENTS.captures(c);

                    let parsed = match parsed {
                        None => return None,
                        Some(v) => v
                    };

                    let quantity = parsed.name("quantity").unwrap().as_str().parse().unwrap();
                    let color = parsed.name("color").unwrap().as_str();
                    Some((quantity, color.to_owned()))
                })
                .collect::<Vec<_>>();

            (color.as_str().to_owned(), contents)
        })
        .collect()
}

#[aoc(day7, part1)]
pub fn day7_part1(input: &HashMap<String, Vec<(usize, String)>>) -> usize {
    let mut look_for = vec!["shiny gold"];
    let mut colors = HashSet::new();

    while let Some(color) = look_for.pop() {
        let more_colors: HashSet<_> = input.iter()
            .filter(|(_, c)| {
                c.iter().any(|(q, c)| q > &0 && c == color)
            })
            .map(|(k, _)| k)
            .collect();

        let new_colors: Vec<_> = more_colors.difference(&colors).map(|c| c.as_str()).collect();
        colors = colors.union(&more_colors).map(|c| *c).collect();
        look_for.extend_from_slice(&new_colors[..]);
    }
    colors.len()
}


#[aoc(day7, part2)]
pub fn day7_part2(input: &HashMap<String, Vec<(usize, String)>>) -> usize {
    let mut look_for = vec![(1, String::from("shiny gold"))];
    let mut sum = 0;

    while let Some((quantity, color)) = look_for.pop() {
        sum += quantity;
        look_for.extend_from_slice(
            &input.get(&color)
            .unwrap_or(&vec![])
            .iter()
            .map(|(q, v)| {
                (quantity * q, v.to_owned())
            })
            .collect::<Vec<_>>()[..]
        );
    }
    sum - 1
}
