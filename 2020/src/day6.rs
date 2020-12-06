use std::collections::HashSet;

#[aoc(day6, part1)]
pub fn part_one(input: &str) -> usize {
    input
        .split("\n\n")
        .map(|ch| {
            ch.split("\n")
                .flat_map(|ch| ch.chars())
                .collect::<HashSet<_>>()
                .len()
        })
        .fold(0, |n, l| n + l)
}

#[aoc(day6, part2)]
pub fn part_two(input: &str) -> usize {
    input
        .split("\n\n")
        .map(|ch| {
            ch.split("\n")
                .map(|ch| ch.chars().collect::<HashSet<_>>())
                .fold(None, |set: Option<HashSet<_>>, chars| match set {
                    Some(set) => Some(set.intersection(&chars).copied().collect::<HashSet<_>>()),
                    None => Some(chars),
                })
                .unwrap_or_default()
                .len()
        })
        .fold(0, |n, l| n + l)
}
