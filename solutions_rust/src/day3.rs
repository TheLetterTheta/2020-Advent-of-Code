use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day3)]
pub fn input_generator(input: &str) -> Vec<Vec<bool>> {
    input.lines()
        .map(|l| {
            l.chars().map(|c| c == '#').collect()
        })
        .collect()
}

#[aoc(day3, part1)]
pub fn part_1(input: &[Vec<bool>]) -> usize {
    input.iter()
        .enumerate()
        .filter(|(k, l)| {
            l[(3*k) % l.len()]
        })
        .count()
}


#[aoc(day3, part2)]
pub fn part_2(input: &[Vec<bool>]) -> usize {
    let sums = input.into_iter()
        .enumerate()
        .fold((0, 0, 0, 0, 0), |acc, (k, l)| {
            ( acc.0 + if l[k % l.len()] { 1 } else { 0 }
            , acc.1 + if l[(3*k) % l.len()] { 1 } else { 0 }
            , acc.2 + if l[(5*k) % l.len()] { 1 } else { 0 }
            , acc.3 + if l[(7*k) % l.len()] { 1 } else { 0 }

            , if k % 2 == 0 {
                acc.4 + if l[(k/ 2) % l.len()] { 1 } else { 0 }
            }
            else {
                acc.4
            }
            )
        });
    
    sums.0 * sums.1 * sums.2 * sums.3 * sums.4
}

