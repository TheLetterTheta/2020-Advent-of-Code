use aoc_runner_derive::{aoc, aoc_generator};

pub fn string_to_input(input: &str) -> Result<bool, &'static str> {
    let mut byr: Option<bool> = None;
    let mut iyr: Option<bool> = None;
    let mut eyr: Option<bool> = None;
    let mut hgt: Option<bool> = None;
    let mut hcl: Option<bool> = None;
    let mut ecl: Option<bool> = None;
    let mut pid: Option<bool> = None;
    input.split(char::is_whitespace).for_each(|i| {
        let split: Vec<&str> = i.splitn(2, ':').collect();
        let prefix: &str = split[0];
        let suffix: &str = split[1];

        match prefix {
            "byr" => {
                let year: u16 = suffix.parse().unwrap();
                if year >= 1920 && year <= 2002 {
                    byr = Some(true);
                } else {
                    byr = Some(false);
                }
            }
            "iyr" => {
                let year: u16 = suffix.parse().unwrap();
                if year >= 2010 && year <= 2020 {
                    iyr = Some(true);
                } else {
                    iyr = Some(false);
                }
            }
            "eyr" => {
                let year: u16 = suffix.parse().unwrap();
                if year >= 2020 && year <= 2030 {
                    eyr = Some(true);
                } else {
                    eyr = Some(false);
                }
            }
            "hgt" => {
                let prefix = suffix
                    .chars()
                    .take_while(|&c| char::is_numeric(c))
                    .collect::<String>();
                let suffix = suffix.strip_prefix(&prefix).unwrap();
                let prefix: u16 = prefix.parse().unwrap();

                if (suffix == "in" && prefix >= 59 && prefix <= 76)
                    || (suffix == "cm" && prefix >= 150 && prefix <= 193)
                {
                    hgt = Some(true)
                } else {
                    hgt = Some(false)
                }
            }
            "hcl" => {
                if suffix.chars().nth(0).unwrap() == '#'
                    && suffix.chars().skip(1).all(|c| char::is_ascii_hexdigit(&c))
                {
                    hcl = Some(true);
                } else {
                    hcl = Some(false);
                }
            }
            "ecl" => match suffix {
                "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth" => ecl = Some(true),
                _ => ecl = Some(false),
            },
            "pid" => {
                if suffix.len() == 9 {
                    pid = Some(true)
                } else {
                    pid = Some(false)
                }
            }
            _ => {}
        }
    });

    if byr.is_none()
        || iyr.is_none()
        || eyr.is_none()
        || hgt.is_none()
        || hcl.is_none()
        || ecl.is_none()
        || pid.is_none()
    {
        return Err("Missing required field");
    } else {
        return Ok(byr.unwrap()
            && iyr.unwrap()
            && eyr.unwrap()
            && hgt.unwrap()
            && hcl.unwrap()
            && ecl.unwrap()
            && pid.unwrap());
    }
}

#[aoc_generator(day4)]
pub fn input_generator(input: &str) -> Vec<Result<bool, &'static str>> {
    input.split("\n\n").map(|l| string_to_input(l)).collect()
}

#[aoc(day4, part1)]
pub fn part_1(input: &[Result<bool, &'static str>]) -> usize {
    input.iter().filter(|l| l.is_ok()).count()
}

#[aoc(day4, part2)]
pub fn part_2(input: &[Result<bool, &'static str>]) -> usize {
    input
        .iter()
        .filter(|i| i.is_ok())
        .map(|i| i.unwrap())
        .filter(|&i| i)
        .count()
}
