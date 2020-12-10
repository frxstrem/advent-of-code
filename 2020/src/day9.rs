use std::ops::Add;

const PREAMBLE_SIZE: usize = 25;

#[aoc_generator(day9)]
pub fn input_generator(input: &str) -> Vec<i64> {
    input
        .lines()
        .map(str::parse)
        .collect::<Result<_, _>>()
        .unwrap()
}

#[aoc(day9, part1)]
pub fn part_one(input: &[i64]) -> i64 {
    find_first_invalid(input).0
}

#[aoc(day9, part2)]
pub fn part_two(input: &[i64]) -> i64 {
    let (n, i) = find_first_invalid(input);

    let sums = any_contiguous_sum_to(n, &input[..i]).unwrap();

    let min = sums.iter().min().unwrap();
    let max = sums.iter().max().unwrap();

    min + max
}

fn find_first_invalid(input: &[i64]) -> (i64, usize) {
    for i in PREAMBLE_SIZE..input.len() {
        let n = input[i];
        let previous = &input[(i - PREAMBLE_SIZE)..i];

        if any_sum_to(n, previous).is_none() {
            return (n, i);
        }
    }

    panic!()
}

fn any_sum_to<T: Copy + Eq + Add<Output = T>>(n: T, range: &[T]) -> Option<(T, T)> {
    for i in 0..range.len() {
        for j in 0..range.len() {
            if i == j || range[i] == range[j] {
                continue;
            }

            if range[i] + range[j] == n {
                return Some((range[i], range[j]));
            }
        }
    }

    None
}

fn any_contiguous_sum_to<T: Copy + Default + Eq + Add<Output = T>>(
    n: T,
    range: &[T],
) -> Option<&[T]> {
    for i in 0..range.len() {
        if i + 2 >= range.len() {
            break;
        }

        for j in (i + 2)..range.len() {
            let sum = range[i..j].iter().fold(T::default(), |c, &n| c + n);

            if sum == n {
                return Some(&range[i..j]);
            }
        }
    }

    None
}
