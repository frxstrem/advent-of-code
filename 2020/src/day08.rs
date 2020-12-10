use std::collections::{HashMap, HashSet, VecDeque};
use std::convert::TryFrom;
use std::str::FromStr;

#[derive(Copy, Clone)]
pub enum Instr {
    Acc(isize),
    Jmp(isize),
    Nop(isize),
}

impl FromStr for Instr {
    type Err = Box<dyn std::error::Error>;

    fn from_str(s: &str) -> Result<Instr, Box<dyn std::error::Error>> {
        let mut it = s.splitn(2, " ");
        let op = it.next().ok_or_else(|| "expected string")?;
        let n = it.next().ok_or_else(|| "expected string")?.parse()?;
        if it.next() != None {
            return Err("unexpected string".into());
        }

        match op {
            "acc" => Ok(Instr::Acc(n)),
            "jmp" => Ok(Instr::Jmp(n)),
            "nop" => Ok(Instr::Nop(n)),
            _ => Err(format!("unknown: {}", op).into()),
        }
    }
}

#[aoc_generator(day8)]
pub fn input_generator(input: &str) -> Vec<Instr> {
    input
        .lines()
        .map(str::parse)
        .collect::<Result<_, _>>()
        .unwrap()
}

#[aoc(day8, part1)]
pub fn parse_one(input: &[Instr]) -> isize {
    run(&input)
}

#[aoc(day8, part2)]
pub fn parse_two(input: &[Instr]) -> isize {
    // for this part, consider "acc X" equivalent to "jmp +1"

    // find list of instructions reachable from the first
    let mut start_reachable = HashSet::<isize>::new();

    {
        let mut pc: isize = 0;

        loop {
            if start_reachable.contains(&pc) {
                break;
            }

            start_reachable.insert(pc);

            match input[usize::try_from(pc).unwrap()] {
                Instr::Acc(_) | Instr::Nop(_) => pc += 1,
                Instr::Jmp(n) => pc += n as isize,
            }
        }
    }

    // for each instruction, find all instructions that can reach it
    let mut reverse_map = HashMap::<isize, Vec<isize>>::new();

    for (pos, instr) in input.iter().enumerate() {
        let pos = pos as isize;
        let target = match instr {
            Instr::Acc(_) | Instr::Nop(_) => pos + 1,
            Instr::Jmp(n) => pos + n,
        };

        reverse_map.entry(target).or_default().push(pos)
    }

    // find all instructions that will reach the end
    let mut end_reachable = HashSet::<isize>::new();

    {
        let mut queue = VecDeque::new();
        queue.push_back(input.len() as isize);

        while let Some(n) = queue.pop_front() {
            if let Some(list) = reverse_map.get(&n) {
                end_reachable.extend(list);
                queue.extend(list);
            }
        }
    }

    // find an instruction reachable from the start that, when swapped, will
    // reach the end

    let changed = find_changed(input, &start_reachable, &end_reachable);

    let mut input = input.to_vec();
    input[changed] = match input[changed] {
        Instr::Acc(n) => unreachable!(),
        Instr::Jmp(n) => Instr::Nop(n),
        Instr::Nop(n) => Instr::Jmp(n),
    };

    run(&input)
}

fn run(input: &[Instr]) -> isize {
    let mut visited = HashSet::<isize>::new();

    let mut pc: isize = 0;
    let mut value = 0;

    loop {
        if pc == input.len() as isize || visited.contains(&pc) {
            return value;
        }

        visited.insert(pc);

        match input[usize::try_from(pc).unwrap()] {
            Instr::Acc(n) => {
                value += n;
                pc += 1;
            }

            Instr::Jmp(n) => {
                pc += n as isize;
            }

            Instr::Nop(n) => {
                pc += 1;
            }
        }
    }
}

fn find_changed(
    input: &[Instr],
    start_reachable: &HashSet<isize>,
    end_reachable: &HashSet<isize>,
) -> usize {
    for &n in start_reachable {
        match &input[usize::try_from(n).unwrap()] {
            Instr::Acc(_) => {
                // cannot be changed!
                continue;
            }

            Instr::Jmp(_) => {
                if end_reachable.contains(&(n + 1)) {
                    return usize::try_from(n).unwrap();
                }
            }

            Instr::Nop(k) => {
                if end_reachable.contains(&(n + k)) {
                    return usize::try_from(n).unwrap();
                }
            }
        }
    }

    panic!()
}
