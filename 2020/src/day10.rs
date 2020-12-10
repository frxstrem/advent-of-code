use itertools::Itertools;
use std::hint::unreachable_unchecked;

#[aoc_generator(day10)]
pub fn input_generator(input: &str) -> Vec<usize> {
    input
        .lines()
        .map(|line| line.parse().unwrap())
        .sorted()
        .collect()
}

#[aoc(day10, part1)]
pub fn part_one(input: &[usize]) -> usize {
    let (c1, c3) = std::iter::once(&0)
        .chain(input)
        .tuple_windows()
        .map(|(x, y)| y - x)
        .chain(std::iter::once(3))
        .fold((0, 0), |(c1, c3), n| match n {
            1 => (c1 + 1, c3),
            2 => (c1, c3),
            3 => (c1, c3 + 1),
            _ => panic!(),
        });
    c1 * c3
}

#[aoc(day10, part2)]
pub fn part_two(input: &[usize]) -> usize {
    unsafe {
        let mut it = input.iter().rev().copied();

        let mut last = match it.next() {
            Some(n) => n,
            None => unreachable_unchecked(),
        };
        let mut s = [1, 0, 0];

        while let Some(n) = it.next() {
            s = match last - n {
                1 => [s[0] + s[1] + s[2], s[0], s[1]],
                2 => [s[0] + s[1], 0, s[0]],
                3 => [s[0], 0, 0],
                _ => unreachable_unchecked(),
            };
            last = n;
        }

        s[0] + s[1] + s[2]
    }
}
