use std::fs::read_to_string;

fn main() {
    // parse inputs
    let input = read_to_string("input.txt").unwrap();
    let mut input = input
        .split("\n")
        .map(str::parse)
        .collect::<Result<Vec<i32>, _>>()
        .unwrap();

    // sort inputs
    input.sort();

    part_one(&input);
    part_two(&input);
}

fn part_one(input: &[i32]) {
    let (a, b) = find_two_summing_to(input, 2020).expect("not found");
    println!("answer 1: {} * {} = {}", a, b, a * b);
}

fn part_two(input: &[i32]) {
    const SUM_TARGET: i32 = 2020;

    for (i, a) in input.iter().enumerate() {
        if let Some((b, c)) = find_two_summing_to(&input[i..], SUM_TARGET - a) {
            println!("answer 2: {} * {} * {} = {}", a, b, c, a * b * c);
            return;
        }
    }

    panic!("not found")
}

fn find_two_summing_to(input: &[i32], target: i32) -> Option<(i32, i32)> {
    // find half of sum target
    let index = input.binary_search(&(target / 2)).unwrap_or_else(|i| i);

    // expand left and right until a matching sum is found
    let mut left = index;
    let mut right = index;

    loop {
        let sum = input[left] + input[right];

        if sum > target {
            if left == 0 {
                break;
            }
            left -= 1;
            continue;
        } else if sum < target {
            if right == input.len() - 1 {
                break;
            }
            right += 1;
            continue;
        }

        return Some((input[left], input[right]));
    }

    None
}
