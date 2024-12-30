#![feature(portable_simd)]
use rayon::prelude::*;
use std::{ops::BitAnd, simd::prelude::*};

advent_of_code::solution!(2);

fn parse(input: &str) -> ([[i8; 8]; 1000], usize) {
    let mut nums = [[0 as i8; 8]; 1000];

    let lines: Vec<_> = input.par_lines().collect();

    let len = lines.len();

    for (row, line) in lines.into_iter().enumerate() {
        let mut col = 0;
        for chars in line.split(' ') {
            let n: i8 = chars.parse().unwrap_or_default();
            nums[row][col] = n;
            col += 1;
        }
    }

    (nums, len)
}

fn row_is_safe(row: Simd<i8, 8>) -> bool {
    let zeros = Simd::<i8, 8>::splat(0);

    let offset = simd_swizzle!(row, zeros, [1, 2, 3, 4, 5, 6, 7, 8]);

    let mask = offset.simd_ne(zeros);
    let row = mask.select(row, zeros);

    let diffs = row - offset;
    let abs = diffs.abs();

    let increasing = diffs.simd_eq(abs).all();
    let decreasing = diffs.simd_eq(-abs).all();

    // if any differences are 0, fail, ignoring masked vals
    let diffs_all_nonzero = (abs.simd_eq(zeros) ^ mask).all();

    // if any differences are > 3, fail
    let diffs_all_lt_4 = !(abs.simd_gt(Simd::<i8, 8>::splat(3)).any());

    // bad if any diff is outside of 1-3, or if not all increasing or decreasing
    let good = diffs_all_nonzero & diffs_all_lt_4 & (increasing | decreasing);

    good
}

fn naieve_pt1(nums: [[i8; 8]; 1000], row: usize) -> u32 {
    let mut safe_count = 0;
    'outer: for row in &nums[..row] {
        let increasing = row[0] < row[1];

        'inner: for idx in 0..7 {
            let a = row[idx];
            let b = row[idx + 1];

            if b == 0 {
                break 'inner;
            };

            if (a < b) != increasing {
                continue 'outer;
            };

            let diff = a.abs_diff(b);

            if (diff == 0) | (diff > 3) {
                continue 'outer;
            };
        }

        safe_count += 1;
    }

    safe_count
}

pub fn part_one(input: &str) -> Option<u32> {
    let (nums, row) = parse(input);

    Some(naieve_pt1(nums, row))

    // let mut sum = 0;
    // for row in &nums[..row] {
    //     let simd_row = Simd::<i8, 8>::load_or_default(row);
    //     let safe = row_is_safe(simd_row);
    //     sum += safe as u32;
    // }

    // Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    let (nums, row) = parse(input);
    let zeros = Simd::<i8, 8>::splat(0);

    let mut sum = 0;
    for row in &nums[..row] {
        let simd_row = Simd::<i8, 8>::load_or_default(row);
        let safe = row_is_safe(simd_row);

        let o0 = row_is_safe(simd_swizzle!(simd_row, zeros, [1, 2, 3, 4, 5, 6, 7, 8]));

        let o1 = row_is_safe(simd_swizzle!(simd_row, zeros, [0, 2, 3, 4, 5, 6, 7, 8]));

        let o2 = row_is_safe(simd_swizzle!(simd_row, zeros, [0, 1, 3, 4, 5, 6, 7, 8]));

        let o3 = row_is_safe(simd_swizzle!(simd_row, zeros, [0, 1, 2, 4, 5, 6, 7, 8]));

        let o4 = row_is_safe(simd_swizzle!(simd_row, zeros, [0, 1, 2, 3, 5, 6, 7, 8]));

        let o5 = row_is_safe(simd_swizzle!(simd_row, zeros, [0, 1, 2, 3, 4, 6, 7, 8]));

        let o6 = row_is_safe(simd_swizzle!(simd_row, zeros, [0, 1, 2, 3, 4, 5, 7, 8]));

        let o7 = row_is_safe(simd_swizzle!(simd_row, zeros, [0, 1, 2, 3, 4, 5, 6, 8]));

        let any = o0 | o1 | o2 | o3 | o4 | o5 | o6 | o7 | safe;

        sum += any as u32;
    }

    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));
    }
}
