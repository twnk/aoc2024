#![feature(portable_simd)]
#![feature(iter_collect_into)]
use std::simd::prelude::*;

advent_of_code::solution!(5);

#[cfg(test)]
const NUM_RULES: usize = 21;

#[cfg(not(test))]
const NUM_RULES: usize = 1176;

const RULES_ALLOCATION: usize = (NUM_RULES * 2).next_power_of_two();

#[inline]
fn parse_rules_chunk(chunk: &[u8]) -> u8x16 {
    let (left, right) = {
        // with 3 vectors of 16 u8 chars, we will extract the 32 digit chars
        // into two vectors
        let start = u8x16::load_or_default(&chunk[0..16]);
        let middle = u8x16::load_or_default(&chunk[16..32]);
        let end = u8x16::load_or_default(&chunk[32..48]);

        const LEFT_SELECTOR: [usize; 16] = [
            // 16 bytes of start here
            00, 01, // |
            03, 04, // \n
            06, 07, // |
            09, 10, // \n
            12, 13, // |
            15, // first 8 bytes of middle here
            16, // \n
            18, 19, // |
            21, 22, // \n
        ];

        const RIGHT_SELECTOR: [usize; 16] = [
            // last 8 bytes of middle here
            08, 09, // \n
            11, 12, // |
            14, 15, // \n
            // 16 bytes of end here
            17, 18, // |
            20, 21, // \n
            23, 24, // |
            26, 27, // \n
            29, 30, // |
        ];

        (
            simd_swizzle!(start, middle, LEFT_SELECTOR),
            simd_swizzle!(middle, end, RIGHT_SELECTOR),
        )
    };

    const ONES: [usize; 16] = [1, 3, 5, 7, 9, 11, 13, 15, 17, 19, 21, 23, 25, 27, 29, 31];

    const TENS: [usize; 16] = [0, 2, 4, 6, 8, 10, 12, 14, 16, 18, 20, 22, 24, 26, 28, 30];

    // to convert from ascii subtract 48 from each digit
    let forty_eight = u8x16::splat(48);

    let ones = simd_swizzle!(left, right, ONES) - forty_eight;
    let tens = (simd_swizzle!(left, right, TENS) - forty_eight) * u8x16::splat(10);
    let nums = ones + tens;

    nums
}

fn parse_rules(input: &str) -> [u8; RULES_ALLOCATION] {
    // each line is 6 u8s
    // we only want 4 u8s as chars from it
    // so we want to operate on 4 lines
    // which we read as 4 * 6 bytes = 24
    //
    // but then we contract each pair of chars to 1 u8
    // so we can fill 8 lines into single vector of u8s

    //

    // nn|nn.

    let mut collector = [0; RULES_ALLOCATION];

    let input = input.as_bytes();

    let rules = &input[..NUM_RULES * 6];

    let mut chunks = rules.chunks_exact(48);

    let mut slice_idx: usize = 0;

    for chunk in &mut chunks {
        let nums = parse_rules_chunk(chunk);

        nums.copy_to_slice(&mut collector[slice_idx..slice_idx + 16]);
        slice_idx += 16;
    }

    let mut padding = [b'0'; 48];
    let remainder = chunks.remainder();
    padding[..remainder.len()].copy_from_slice(remainder);

    parse_rules_chunk(&padding).copy_to_slice(&mut collector[slice_idx..slice_idx + 16]);

    collector
}

#[cfg(test)]
const NUM_CASES: usize = 6;

#[cfg(not(test))]
const NUM_CASES: usize = 194;

const CASES_MAX: usize = 23;

// fn parse_cases(input: &str) -> [[u8; CASES_MAX]; NUM_CASES] {
//     let mut cases = [[0; CASES_MAX]; NUM_CASES];

//     let input = input.as_bytes();

//     let rules = &input[NUM_RULES * 6 + 1..];

//     let

//     cases
// }

#[inline]
fn parse_case(acc: &mut Vec<u8>, line: &str) -> () {
    acc.clear();
    line.split(',')
        .map(|n| n.parse::<u8>().unwrap_or_default())
        .collect_into(acc);
}

#[inline]
fn evaluate_case(pages: &[u8], rules: &[u8; RULES_ALLOCATION]) -> Option<(usize, usize)> {
    for (page_idx, page) in pages.iter().enumerate() {
        for (rule_idx, rule_part) in rules.iter().enumerate() {
            if rule_part == page {
                // first half of rule means 2nd half must be AFTER
                if rule_idx % 2 == 0 {
                    // let rule_other = rules[rule_idx.saturating_add(1)];

                    // for page in &pages[page_idx..] {
                    //     if rule_other == *page {
                    //         // constraint matched
                    //         println!("page: {} must be after: {} matched, continuing", rule_part, rule_other);
                    //         continue 'loop_pages;
                    //     }
                    // }
                } else {
                    // second half of rule means 1st half must be BEFORE
                    let rule_other = rules[rule_idx.saturating_sub(1)];

                    for (page_cmp_idx, page) in (&pages[page_idx..]).iter().enumerate() {
                        if rule_other == *page {
                            println!(
                                "page: {} must be before: {} violated, breaking",
                                rule_part, rule_other
                            );
                            // violation found
                            return Some((page_idx, page_idx + page_cmp_idx));
                        }
                    }
                }
            }
        }
    }

    None
}

pub fn part_one(input: &str) -> Option<u32> {
    let rules = parse_rules(input);
    // println!("{:?}", rules);
    let cases = &input[NUM_RULES * 6 + 1..];

    let mut result: u32 = 0;
    let mut pages: Vec<u8> = Vec::with_capacity(CASES_MAX);

    for case in cases.lines() {
        parse_case(&mut pages, case);

        if let None = evaluate_case(&pages, &rules) {
            let middle = pages[pages.len() / 2];
            println!("passed all rules: {:?} middle: {}", pages, middle);
            result += middle as u32;
        }
    }

    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    let rules = parse_rules(input);
    // println!("{:?}", rules);
    let cases = &input[NUM_RULES * 6 + 1..];

    let mut result: u32 = 0;
    let mut pages: Vec<u8> = Vec::with_capacity(CASES_MAX);
    for (n, case) in cases.lines().enumerate() {
        parse_case(&mut pages, case);

        let mut passing = true;

        println!("{} {}", n, case);

        while let Some((swap_idx, swap_other_idx)) = evaluate_case(&pages, &rules) {
            passing = false;
            println!(
                "{:?} failed, swapping: {} {}",
                pages, pages[swap_idx], pages[swap_other_idx]
            );
            pages.swap(swap_idx, swap_other_idx);
        }

        if !passing {
            let middle = pages[pages.len() / 2];
            println!("passed all rules: {:?} middle: {}", pages, middle);
            result += middle as u32;
        }
    }
    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_chunk_full() {
        let input1 = "47|53\n97|13\n97|61\n97|47\n75|29\n61|13\n75|53\n29|13\n".as_bytes();
        let input2 = "97|29\n53|29\n61|53\n97|53\n61|29\n47|13\n75|47\n97|75\n".as_bytes();

        assert_eq!(
            parse_rules_chunk(input1),
            u8x16::from_array([47, 53, 97, 13, 97, 61, 97, 47, 75, 29, 61, 13, 75, 53, 29, 13])
        );

        assert_eq!(
            parse_rules_chunk(input2),
            u8x16::from_array([97, 29, 53, 29, 61, 53, 97, 53, 61, 29, 47, 13, 75, 47, 97, 75])
        );
    }

    #[test]
    fn test_parse_chunk_partial() {
        let input = "47|61\n75|61\n47|29\n75|13\n53|13\n".as_bytes();

        let mut padding = [b'0'; 48];
        padding[..input.len()].copy_from_slice(input);

        assert_eq!(
            parse_rules_chunk(&padding),
            u8x16::from_array([47, 61, 75, 61, 47, 29, 75, 13, 53, 13, 00, 00, 00, 00, 00, 00])
        );
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(143));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(123));
    }
}
