struct Map(Vec<Vec<bool>>);

impl Map {
    fn height(&self) -> usize {
        self.0.len()
    }

    fn get(&self, x: usize, y: usize) -> bool {
        if y >= self.0.len() {
            panic!("out of bounds");
        }

        let b = self.0[y][x % self.0[y].len()];
        eprintln!("getting {:?}: {:?}", (x, y), b);
        b
    }

    fn slope(&self, dx: usize, dy: usize) -> impl '_ + Iterator<Item = bool> {
        (0..self.height())
            .step_by(dy)
            .enumerate()
            .map(move |(i, y)| self.get(i * dx, y))
    }
}

fn main() {
    // read input
    let input = Map(include_str!("input.txt")
        .lines()
        .map(|s| s.chars().map(|ch| ch == '#').collect::<Vec<_>>())
        .collect::<Vec<_>>());

    // part one
    println!("{}", input.slope(3, 1).filter(|x| *x).count());

    // part two
    let slopes = &[(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];

    let o = slopes
        .iter()
        .copied()
        .map(|(x, y)| input.slope(x, y).filter(|x| *x).count())
        .fold(1, |c, n| c * n);

    println!("{}", o);
}
