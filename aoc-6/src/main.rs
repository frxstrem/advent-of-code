use std::collections::HashSet;

fn main() {
    // part 1
    println!(
        "{}",
        include_str!("input.txt")
            .split("\n\n")
            .map(|ch| {
                ch.split("\n")
                    .flat_map(|ch| ch.chars())
                    .collect::<HashSet<_>>()
                    .len()
            })
            .fold(0, |n, l| n + l)
    );

    // part 2
    println!(
        "{}",
        include_str!("input.txt")
            .split("\n\n")
            .map(|ch| {
                ch.split("\n")
                    .map(|ch| ch.chars().collect::<HashSet<_>>())
                    .fold(None, |set: Option<HashSet<_>>, chars| match set {
                        Some(set) => {
                            Some(set.intersection(&chars).copied().collect::<HashSet<_>>())
                        }
                        None => Some(chars),
                    })
                    .unwrap_or_default()
                    .len()
            })
            .fold(0, |n, l| n + l)
    );
}
