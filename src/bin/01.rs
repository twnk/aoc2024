use std::collections::HashMap;

use rayon::prelude::*;

advent_of_code::solution!(1);

fn input_to_vecs(input: &str) -> (Vec<u32>, Vec<u32>) {
    let mut left: Vec<u32> = Vec::with_capacity(1000);
    let mut right: Vec<u32> = Vec::with_capacity(1000);

    let lines = input.par_lines().collect::<Vec<_>>();

    lines
        .into_par_iter()
        .map(|l| {
            let (n1, n2) = l.split_once("   ").unwrap_or_default();
            let n1: u32 = n1.parse().unwrap_or_default();
            let n2: u32 = n2.parse().unwrap_or_default();

            (n1, n2)
        })
        .unzip_into_vecs(&mut left, &mut right);

    (left, right)
}

pub fn part_one(input: &str) -> Option<u32> {
    let (mut left, mut right) = input_to_vecs(input);

    left.par_sort_unstable();
    right.par_sort_unstable();

    let sum = left
        .into_par_iter()
        .zip_eq(right)
        .map(|(l, r)| l.abs_diff(r))
        .sum::<u32>();

    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    let (mut left, mut right) = input_to_vecs(input);

    let mut scores: HashMap<u32, (u8, u8)> = HashMap::with_capacity(1000);

    left.par_sort_unstable();
    right.par_sort_unstable();

    for i in left {
        scores
            .entry(i)
            .and_modify(|(l, _)| *l += 1)
            .or_insert((1, 0));
    }

    for i in right {
        scores
            .entry(i)
            .and_modify(|(_, r)| *r += 1)
            .or_insert((0, 1));
    }

    let sum = scores
        .into_par_iter()
        .map(|(i, (l, r))| i * (l * r) as u32)
        .sum();

    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(31));
    }
}
