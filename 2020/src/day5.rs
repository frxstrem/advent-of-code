#[aoc_generator(day5)]
pub fn input_generator(input: &str) -> Vec<i32> {
    input
        .split("\n")
        .map(|line| {
            line.chars().fold(0, |n, ch| {
                2 * n + if ch == 'B' || ch == 'R' { 1 } else { 0 }
            })
        })
        .collect::<Vec<i32>>()
}

#[aoc(day5, part1)]
pub fn part_one(input: &[i32]) -> i32 {
    input.iter().copied().max().unwrap()
}

#[aoc(day5, part2)]
pub fn part_two(input: &[i32]) -> i32 {
    let lowest = input.iter().copied().min().unwrap();
    let highest = input.iter().copied().max().unwrap();

    let missing = (lowest..=highest)
        .into_iter()
        .filter(|n| !input.contains(n))
        .collect::<Vec<i32>>();

    missing[0]
}
