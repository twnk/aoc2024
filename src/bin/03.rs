advent_of_code::solution!(3);
use regex::Regex;

pub fn part_one(input: &str) -> Option<u32> {
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();

    let mut sum = 0;
    for c in re.captures_iter(input) {
        if let (Some(a), Some(b)) = (c.get(1), c.get(2)) {
            let a_int = a.as_str().parse::<u32>().unwrap();
            let b_int = b.as_str().parse::<u32>().unwrap();

            sum += a_int * b_int;
        };
    }

    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    let re = Regex::new(r"(don't\(\))|(do\(\))|mul\((\d+),(\d+)\)").unwrap();

    let mut sum = 0;
    let mut doing = true;

    for c in re.captures_iter(input) {
        if let Some(_) = c.get(1) {
            doing = false;
        } else if let Some(_) = c.get(2) {
            doing = true;
        } else if doing {
            if let (Some(a), Some(b)) = (c.get(3), c.get(4)) {
                let a_int = a.as_str().parse::<u32>().unwrap();
                let b_int = b.as_str().parse::<u32>().unwrap();

                sum += a_int * b_int;
            }
        };
    }

    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(161));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(48));
    }
}
