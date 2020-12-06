use std::collections::HashMap;

const REQUIRED: &[(&str, fn(&str) -> Option<bool>)] = &[
    // byr (Birth Year) - four digits; at least 1920 and at most 2002.
    ("byr", |s| {
        let n = s.parse::<i32>().ok()?;
        Some(n >= 1920 && n <= 2002)
    }),
    // iyr (Issue Year) - four digits; at least 2010 and at most 2020.
    ("iyr", |s| {
        let n = s.parse::<i32>().ok()?;
        Some(n >= 2010 && n <= 2020)
    }),
    // eyr (Expiration Year) - four digits; at least 2020 and at most 2030.
    ("eyr", |s| {
        let n = s.parse::<i32>().ok()?;
        Some(n >= 2020 && n <= 2030)
    }),
    // hgt (Height) - a number followed by either cm or in:
    // If cm, the number must be at least 150 and at most 193.
    // If in, the number must be at least 59 and at most 76.
    ("hgt", |s| {
        if s.ends_with("cm") {
            let n = s[..(s.len() - 2)].parse::<i32>().ok()?;
            Some(n >= 150 && n <= 193)
        } else if s.ends_with("in") {
            let n = s[..(s.len() - 2)].parse::<i32>().ok()?;
            Some(n >= 59 && n <= 76)
        } else {
            None
        }
    }),
    // hcl (Hair Color) - a # followed by exactly six characters 0-9 or a-f.
    ("hcl", |s| {
        Some(regex::Regex::new("^#[0-9a-f]{6}$").unwrap().is_match(s))
    }),
    // ecl (Eye Color) - exactly one of: amb blu brn gry grn hzl oth.
    ("ecl", |s| {
        Some(["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].contains(&s))
    }),
    // pid (Passport ID) - a nine-digit number, including leading zeroes.
    ("pid", |s| Some(s.len() == 9 && s.parse::<u64>().is_ok())),
];

#[aoc_generator(day4)]
pub fn input_generator(input: &str) -> Vec<HashMap<String, String>> {
    input
        .split("\n\n")
        .map(|s| {
            s.split("\n")
                .flat_map(|s| s.split(" "))
                .map(|s| {
                    let mut it = s.splitn(2, ":");
                    let key = it.next().unwrap().to_string();
                    let value = it.next().unwrap().to_string();
                    (key, value)
                })
                .collect()
        })
        .collect()
}

#[aoc(day4, part1)]
pub fn part_one(input: &[HashMap<String, String>]) -> usize {
    input
        .iter()
        .filter(|fields| {
            REQUIRED
                .iter()
                .copied()
                .all(|(name, _)| fields.contains_key(name))
        })
        .count()
}

#[aoc(day4, part2)]
pub fn part_two(input: &[HashMap<String, String>]) -> usize {
    input
        .iter()
        .filter(|fields| {
            REQUIRED.iter().copied().all(|(name, matcher)| {
                if let Some(value) = fields.get(name) {
                    matcher(value).unwrap_or(false)
                } else {
                    false
                }
            })
        })
        .count()
}
