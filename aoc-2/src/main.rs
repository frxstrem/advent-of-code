use std::fs::read_to_string;

fn main() {
    // parse inputs
    let input = read_to_string("input.txt").unwrap();
    let input = input
        .split("\n")
        .map(|s| {
            let s = s.splitn(3, " ").collect::<Vec<_>>();
            let a = &*s[0]
                .split("-")
                .map(str::parse)
                .collect::<Result<Vec<usize>, _>>()
                .unwrap();

            let b = s[1].chars().next().unwrap();
            let c = s[2];
            (a[0], a[1], b, c)
        })
        .collect::<Vec<_>>();

    // part one
    let n = input
        .iter()
        .filter(|(x, y, ch, s)| {
            let n = s.chars().filter(|t| t == ch).count();
            n >= *x && n <= *y
        })
        .count();

    println!("{}", n);

    // part two
    let n = input
        .iter()
        .filter(|(x, y, ch, s)| {
            s.chars()
                .enumerate()
                .map(|(i, t)| (i + 1, t))
                .filter(|(i, t)| (i == x || i == y) && t == ch)
                .count()
                == 1
        })
        .count();

    println!("{}", n);
}
