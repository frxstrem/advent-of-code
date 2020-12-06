fn main() {
    let mut input = include_str!("input.txt")
        .split("\n")
        .map(|line| {
            line.chars().fold(0, |n, ch| {
                2 * n + if ch == 'B' || ch == 'R' { 1 } else { 0 }
            })
        })
        .collect::<Vec<i32>>();

    // part 1
    let highest = input.iter().copied().max().unwrap();
    println!("{}", highest);

    // part2
    let lowest = input.iter().copied().min().unwrap();

    let missing = (lowest..=highest)
        .into_iter()
        .filter(|n| !input.contains(n))
        .collect::<Vec<i32>>();
    assert_eq!(missing.len(), 1);
    println!("{}", missing[0]);
}
