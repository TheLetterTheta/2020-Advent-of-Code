use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day3)]
pub fn input_generator(input: &str) -> Vec<String> {
    input.lines()
        .map(|l| {
            l.to_owned()
        })
        .collect()
}

#[aoc(day3, part1)]
pub fn part_1(input: &[String]) -> usize {
    input.iter()
        .enumerate()
        .filter(|(k, l)| {
            l.chars().nth((3*k) % l.len()).unwrap() == '#'
        })
        .count()
}


#[aoc(day3, part2)]
pub fn part_2(input: &[String]) -> usize {
    let sums = input.iter()
        .enumerate()
        .fold((0, 0, 0, 0, 0), |acc, (k, l)| {
            ( acc.0 + if l.chars().nth(k % l.len()).unwrap() == '#' { 1 } else { 0 }
            , acc.1 + if l.chars().nth((3*k) % l.len()).unwrap() == '#' { 1 } else { 0 }
            , acc.2 + if l.chars().nth((5*k) % l.len()).unwrap() == '#' { 1 } else { 0 }
            , acc.3 + if l.chars().nth((7*k) % l.len()).unwrap() == '#' { 1 } else { 0 }

            , if k % 2 == 0 {
                acc.4 + if l.chars().nth((k/ 2) % l.len()).unwrap() == '#' { 1 } else { 0 }
            }
            else {
                acc.4
            }
            )
        });
    
    sums.0 * sums.1 * sums.2 * sums.3 * sums.4
}

