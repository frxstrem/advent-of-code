use itertools::Itertools;

#[aoc_generator(day10)]
pub fn input_generator(input: &str) -> Vec<usize> {
    input.lines().map(|line| line.parse().unwrap()).collect()
}

#[aoc(day10, part1)]
pub fn part_one(input: &[usize]) -> usize {
    let mut input = input.to_vec();
    input.sort();

    let (c1, c3) = std::iter::once(0)
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
    let mut input = input.to_vec();
    input.sort();

    let target = input.iter().copied().max().unwrap() + 3;

    let mut combos = vec![0; target + 1];
    combos[target] = 1;

    input
        .iter()
        .rev()
        .copied()
        .chain(std::iter::once(0))
        .for_each(|n| {
            let c = combos[n + 1] + combos[n + 2] + combos[n + 3];
            combos[n] = c;
        });

    combos[0]
}
