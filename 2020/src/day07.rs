use std::collections::{HashMap, HashSet, VecDeque};

#[derive(Debug)]
pub struct Rule {
    bag: String,
    contains: Vec<(u32, String)>,
}

#[aoc_generator(day7)]
pub fn input_generator(input: &str) -> Vec<Rule> {
    input
        .lines()
        .map(|line| {
            let s = line.splitn(5, " ").collect::<Vec<_>>();
            assert_eq!(s.len(), 5);

            let bag = format!("{} {}", s[0], s[1]);
            let contains = if s[4] == "no other bags." {
                Vec::new()
            } else {
                s[4][..(s[4].len() - 1)]
                    .split(", ")
                    .map(|t| {
                        let t = t.split(" ").collect::<Vec<_>>();
                        assert_eq!(t.len(), 4);
                        let k = t[0].parse().unwrap();
                        (k, format!("{} {}", t[1], t[2]))
                    })
                    .collect()
            };

            Rule { bag, contains }
        })
        .collect()
}

// ========

#[aoc(day7, part1)]
pub fn part_one(input: &[Rule]) -> usize {
    let mut contained_in = HashMap::<&str, Vec<&str>>::new();

    for rule in input {
        for (_, name) in &rule.contains {
            contained_in.entry(name).or_default().push(&rule.bag);
        }
    }

    let mut results = HashSet::<&str>::new();
    let mut queue = VecDeque::<&str>::new();

    queue.push_back("shiny gold");

    while let Some(name) = queue.pop_front() {
        if let Some(next_list) = contained_in.get(name) {
            results.extend(next_list);
            queue.extend(next_list);
        }
    }

    results.len()
}

#[aoc(day7, part2)]
pub fn part_two(input: &[Rule]) -> u32 {
    let contains: HashMap<&str, Vec<(u32, &str)>> = input
        .iter()
        .map(|rule| {
            (
                &*rule.bag,
                rule.contains.iter().map(|(n, bag)| (*n, &**bag)).collect(),
            )
        })
        .collect();

    let mut sum = 0;
    let mut queue = VecDeque::<(u32, &str)>::new();

    queue.push_back((1, "shiny gold"));

    while let Some((n, name)) = queue.pop_front() {
        if let Some(next_list) = contains.get(name) {
            for (m, bag) in next_list {
                sum += n * m;
                queue.push_back((n * m, bag));
            }
        }
    }

    sum
}
