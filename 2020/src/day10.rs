use itertools::Itertools;
use std::hint::unreachable_unchecked;

#[aoc_generator(day10)]
pub fn input_generator(input: &str) -> Vec<usize> {
    input.lines().map(|line| line.parse().unwrap()).collect()
}

#[aoc(day10, part1)]
pub fn part_one(input: &[usize]) -> usize {
    let mut counts = [false; 400];

    let input =
        UniqueCountSorted::sort(input.iter().copied(), &mut counts[..(3 * input.len() + 1)]);

    let (c1, c3) = std::iter::once(0)
        .chain(input.iter())
        .tuple_windows()
        .map(|(x, y)| y - x)
        .chain(std::iter::once(3))
        .fold((0, 0), |(c1, c3), n| match n {
            1 => (c1 + 1, c3),
            2 => (c1, c3),
            3 => (c1, c3 + 1),
            _ => unsafe { unreachable_unchecked() },
        });
    c1 * c3
}

#[aoc(day10, part2)]
pub fn part_two(input: &[usize]) -> usize {
    let mut counts = [false; 400];

    let input =
        UniqueCountSorted::sort(input.iter().copied(), &mut counts[..(3 * input.len() + 1)]);

    let mut it = input.iter().rev();

    let mut last = match it.next() {
        Some(n) => n,
        None => unsafe { unreachable_unchecked() },
    };
    let mut s = (1, 0, 0);

    while let Some(n) = it.next() {
        s = match last - n {
            1 => (s.0 + s.1 + s.2, s.0, s.1),
            2 => (s.0 + s.1, 0, s.0),
            3 => (s.0, 0, 0),
            _ => unsafe { unreachable_unchecked() },
        };
        last = n;
    }

    s.0 + s.1 + s.2
}

struct UniqueCountSorted<'c>(&'c mut [bool]);

impl<'c> UniqueCountSorted<'c> {
    pub fn sort<I: IntoIterator<Item = usize>>(
        iter: I,
        counts: &'c mut [bool],
    ) -> UniqueCountSorted<'c> {
        for it in iter {
            counts[it] = true;
        }
        UniqueCountSorted(counts)
    }

    fn iter(&self) -> Iter<'_, 'c> {
        Iter {
            c: self,
            i: 0,
            ri: self.0.len(),
        }
    }
}

struct Iter<'a, 'c> {
    c: &'a UniqueCountSorted<'c>,
    i: usize,
    ri: usize,
}

impl<'a, 'c> Iterator for Iter<'a, 'c> {
    type Item = usize;

    fn next(&mut self) -> Option<usize> {
        loop {
            if self.i >= self.ri {
                return None;
            }

            let k = self.i;
            self.i += 1;
            if self.c.0[k] {
                return Some(k);
            }
        }
    }
}

impl<'a, 'c> DoubleEndedIterator for Iter<'a, 'c> {
    fn next_back(&mut self) -> Option<usize> {
        loop {
            if self.i >= self.ri {
                return None;
            }

            self.ri -= 1;
            if self.c.0[self.ri] {
                return Some(self.ri);
            }
        }
    }
}

#[cfg(test)]
#[test]
fn test_count_sort() {
    let items = vec![9, 1, 5, 2, 6, 2, 6, 7, 3];

    let items = UniqueCountSorted::sort(items.iter().copied(), 10);

    let iter_items: Vec<_> = items.iter().collect();
    assert_eq!(iter_items, vec![1, 2, 3, 5, 6, 7, 9]);

    let rev_items: Vec<_> = items.iter().rev().collect();
    assert_eq!(rev_items, vec![9, 7, 6, 5, 3, 2, 1]);

    let items = UniqueCountSorted::sort(vec![1, 3, 2, 5], 7);

    let mut it = items.iter();
    assert_eq!(it.next(), Some(1));
    assert_eq!(it.next_back(), Some(5));
    assert_eq!(it.next(), Some(2));
    assert_eq!(it.next_back(), Some(3));
    assert_eq!(it.next(), None);
    assert_eq!(it.next_back(), None);
}
