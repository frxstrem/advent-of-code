#[aoc_generator(day2)]
pub fn input_generator(input: &str) -> Vec<(usize, usize, char, String)> {
    input
        .split("\n")
        .map(|s| {
            let s = s.splitn(3, " ").collect::<Vec<_>>();
            let a = &*s[0]
                .split("-")
                .map(str::parse)
                .collect::<Result<Vec<usize>, _>>()
                .unwrap();

            let b = s[1].chars().next().unwrap();
            let c = s[2].to_string();
            (a[0], a[1], b, c)
        })
        .collect::<Vec<_>>()
}

#[aoc(day2, part1)]
pub fn part_one(input: &[(usize, usize, char, String)]) -> usize {
    input
        .iter()
        .filter(|(x, y, ch, s)| {
            let n = s.chars().filter(|t| t == ch).count();
            n >= *x && n <= *y
        })
        .count()
}

#[aoc(day2, part2)]
pub fn part_two<'a>(input: &[(usize, usize, char, String)]) -> usize {
    input
        .iter()
        .filter(|(x, y, ch, s)| {
            s.chars()
                .enumerate()
                .map(|(i, t)| (i + 1, t))
                .filter(|(i, t)| (i == x || i == y) && t == ch)
                .count()
                == 1
        })
        .count()
}
