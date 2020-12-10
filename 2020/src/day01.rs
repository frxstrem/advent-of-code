#[aoc_generator(day1)]
pub fn input_generator(input: &str) -> Vec<i32> {
    // parse inputs
    let mut input = input
        .split("\n")
        .map(str::parse)
        .collect::<Result<Vec<i32>, _>>()
        .unwrap();

    // sort inputs
    input.sort();

    input
}

#[aoc(day1, part1)]
pub fn part_one(input: &[i32]) -> i32 {
    let (a, b) = find_two_summing_to(input, 2020).expect("not found");
    a * b
}

#[aoc(day1, part2)]
pub fn part_two(input: &[i32]) -> i32 {
    const SUM_TARGET: i32 = 2020;

    for (i, a) in input.iter().enumerate() {
        if let Some((b, c)) = find_two_summing_to(&input[i..], SUM_TARGET - a) {
            return a * b * c;
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
