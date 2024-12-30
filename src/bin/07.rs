#![feature(iter_collect_into)]
advent_of_code::solution!(7);
use rayon::prelude::*;
use itertools::{repeat_n, Itertools};

// total 850 largest target 270043318363553 largest operand 999 largest equation 12
// fn get_stats(input: &str) -> () {
//     let equations = parse(input);

//     let len = equations.len();
//     let (max_target, _) = equations.iter().max_by(|(x, _), (y, _)| x.cmp(y) ).unwrap();
//     let max_sum_val = equations.iter().map(|(_, nums)| nums.iter().max().unwrap()).max().unwrap();
//     let max_sum_len = equations.iter().map(|(_, nums)| nums.len()).max().unwrap();

//     println!("total {} largest target {} largest operand {} largest equation {}", len, max_target, max_sum_val, max_sum_len);
// }

#[cfg(test)]
const INPUT_LEN: usize = 9;

#[cfg(not(test))]
const INPUT_LEN: usize = 850;

fn parse(input: &str) -> Vec<(u64, Vec<u64>)> {
    let mut parsed = Vec::with_capacity(INPUT_LEN);
    let lines: Vec<_> = input.par_lines().collect();

    lines
        .into_par_iter()
        .map(|line| {
            let (target_str, nums_str) = line.split_once(": ").unwrap();
            let target = target_str.parse().unwrap_or_default();
            // println!("{}", nums_str);
            let mut nums = Vec::with_capacity(12);
            nums_str.split(' ').map(|n| n.parse::<u64>().unwrap()).collect_into(&mut nums);

            (target, nums)
        })
        .collect_into_vec(&mut parsed);

    parsed
}

fn unique_combinations<T: PartialEq + Copy + Ord>(len: usize, options: Vec<T>) -> Vec<Vec<T>> {
    // let mut possibilities = Vec::new();

    // for option in options {
    //     for _ in 0..len {
    //         possibilities.push(option)
    //     }
    // }
    
    // let res: Vec<Vec<T>> = options
    //     .into_iter()
    //     .permutations(len)
    //     .dedup()
    //     .collect();

    let iter = options.into_iter();

    let res: Vec<Vec<T>> = repeat_n(iter, len).multi_cartesian_product()
        .collect();

    res
}

fn all_unique_combinations<T: PartialEq + Copy + Ord + std::fmt::Debug>(max: usize, options: Vec<T>) -> Vec<Vec<Vec<T>>> {
    let mut res = Vec::with_capacity(max);

    for len in 0..max {
        // println!("finding combinations for {}", len);
        res.push(unique_combinations(len, options.clone()));
        // println!("{:?}", res[len]);
    }

    res
}

fn permute_ops_for_target_pt1(target: u64, nums: &[u64], combination_spaces: &Vec<Vec<Vec<Combine1>>>) -> Option<u64> {

    let x = combination_spaces[nums.len() - 1]
        .iter()
        .find_map(|s| {
            // println!("combinations ({}): {:?} applied to nums: {:?}", s.len(), s, nums);
            let result = &nums[1..].iter().enumerate().fold(*&nums[0], |acc, (idx, x)| {
                match s.get(idx) {
                    Some(Combine1::Add) => acc + x,
                    Some(Combine1::Mult) => acc * x,
                    None => unreachable!(),
                }
            });

            // println!("target: {} got {} (success? {})", target, result, target == *result);

            if target == *result {
                Some(target)
            } else {
                 None
            }
        });

    x

}

fn permute_ops_for_target_pt2(target: u64, nums: &[u64], combination_spaces: &Vec<Vec<Vec<Combine2>>>) -> Option<u64> {
    // let operations = Vec::with_capacity(3 * (nums.len() - 1));
    

    let x = combination_spaces[nums.len() - 1]
        .iter()
        .find_map(|s| {
            let result = &nums[1..].iter().enumerate().fold(*&nums[0], |acc, (idx, x)| {
                match s.get(idx) {
                    Some(Combine2::Add) => acc + x,
                    Some(Combine2::Mult) => acc * x,
                    Some(Combine2::Concat) => (acc.to_string() + &x.to_string()).parse().unwrap_or_default(),
                    None => unreachable!(),
                }
            });

            if target == *result {
                Some(target)
            } else {
                 None
            }
        });

    x

}

pub fn part_one(input: &str) -> Option<u64> {
    let equations = parse(input);
    let combination_spaces = all_unique_combinations::<Combine1>(12, vec![Combine1::Mult, Combine1::Add]);
    let s = equations
        .into_iter()
        .filter_map(|(target, nums)| permute_ops_for_target_pt1(target, &nums, &combination_spaces))
        .sum();
    
    Some(s)
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
enum Combine1 {
    Mult,
    Add,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
enum Combine2 {
    Mult,
    Add,
    Concat
}

pub fn part_two(input: &str) -> Option<u64> {
    let equations = parse(input);
    let combination_spaces = all_unique_combinations::<Combine2>(12, vec![Combine2::Mult, Combine2::Add, Combine2::Concat]);
    let s = equations
        .into_par_iter()
        .filter_map(|(target, nums)| permute_ops_for_target_pt2(target, &nums, &combination_spaces))
        .sum();
    
    Some(s)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3749));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11387));
    }
}
