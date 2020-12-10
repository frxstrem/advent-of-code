pub struct Map(Vec<Vec<bool>>);

impl Map {
    fn height(&self) -> usize {
        self.0.len()
    }

    fn get(&self, x: usize, y: usize) -> bool {
        if y >= self.0.len() {
            panic!("out of bounds");
        }

        self.0[y][x % self.0[y].len()]
    }

    fn slope(&self, dx: usize, dy: usize) -> impl '_ + Iterator<Item = bool> {
        (0..self.height())
            .step_by(dy)
            .enumerate()
            .map(move |(i, y)| self.get(i * dx, y))
    }
}

#[aoc_generator(day3)]
pub fn input_generator(input: &str) -> Map {
    Map(input
        .lines()
        .map(|s| s.chars().map(|ch| ch == '#').collect::<Vec<_>>())
        .collect::<Vec<_>>())
}

#[aoc(day3, part1)]
pub fn part_one(input: &Map) -> usize {
    input.slope(3, 1).filter(|x| *x).count()
}

#[aoc(day3, part2)]
pub fn part_two(input: &Map) -> usize {
    let slopes = &[(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];

    slopes
        .iter()
        .copied()
        .map(|(x, y)| input.slope(x, y).filter(|x| *x).count())
        .fold(1, |c, n| c * n)
}
