advent_of_code::solution!(4);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Letter {
    X,
    M,
    A,
    S,
}

#[cfg(test)]
const GRID_SIZE: usize = 10;

#[cfg(not(test))]
const GRID_SIZE: usize = 140;
const GRID_MAX: usize = GRID_SIZE - 4;

fn parse(input: &str) -> [[Option<Letter>; GRID_SIZE]; GRID_SIZE] {
    let mut grid = [[None; GRID_SIZE]; GRID_SIZE];

    let mut row = 0;
    let mut col = 0;
    for c in input.chars() {
        grid[row][col] = match c {
            'X' => Some(Letter::X),
            'M' => Some(Letter::M),
            'A' => Some(Letter::A),
            'S' => Some(Letter::S),
            '\n' => {
                row += 1;
                col = 0;
                continue;
            }
            _ => {
                col += 1;
                continue;
            }
        };

        col += 1;
    }
    grid
}

pub fn part_one(input: &str) -> Option<u32> {
    let grid = parse(input);

    let mut xmas = 0;

    for (row_idx, row) in grid.iter().enumerate() {
        // println!("{:?}", row);
        for (col_idx, letter) in row.iter().enumerate() {
            if let Some(Letter::X) = letter {
                // X M A S
                if col_idx <= GRID_MAX
                    && grid[row_idx][col_idx + 1] == Some(Letter::M)
                    && grid[row_idx][col_idx + 2] == Some(Letter::A)
                    && grid[row_idx][col_idx + 3] == Some(Letter::S)
                {
                    xmas += 1;
                    // println!("XMAS");
                }

                // S M A X
                if col_idx >= 3
                    && grid[row_idx][col_idx - 1] == Some(Letter::M)
                    && grid[row_idx][col_idx - 2] == Some(Letter::A)
                    && grid[row_idx][col_idx - 3] == Some(Letter::S)
                {
                    xmas += 1;
                    // println!("SMAX");
                }

                // X
                // M
                // A
                // S
                if row_idx <= GRID_MAX
                    && grid[row_idx + 1][col_idx] == Some(Letter::M)
                    && grid[row_idx + 2][col_idx] == Some(Letter::A)
                    && grid[row_idx + 3][col_idx] == Some(Letter::S)
                {
                    xmas += 1;
                    // println!("X\nM\nA\nS\n")
                }

                // S
                // A
                // M
                // X
                if row_idx >= 3
                    && grid[row_idx - 1][col_idx] == Some(Letter::M)
                    && grid[row_idx - 2][col_idx] == Some(Letter::A)
                    && grid[row_idx - 3][col_idx] == Some(Letter::S)
                {
                    xmas += 1;
                    // println!("S\nA\nM\nX\n")
                }

                // X
                // .M
                // ..A
                // ...S
                if row_idx <= GRID_MAX
                    && col_idx <= GRID_MAX
                    && grid[row_idx + 1][col_idx + 1] == Some(Letter::M)
                    && grid[row_idx + 2][col_idx + 2] == Some(Letter::A)
                    && grid[row_idx + 3][col_idx + 3] == Some(Letter::S)
                {
                    xmas += 1;
                    // println!("X\n.M\n..A\n..S\n");
                }

                // S
                // .A
                // ..M
                // ...X
                if row_idx >= 3
                    && col_idx >= 3
                    && grid[row_idx - 1][col_idx - 1] == Some(Letter::M)
                    && grid[row_idx - 2][col_idx - 2] == Some(Letter::A)
                    && grid[row_idx - 3][col_idx - 3] == Some(Letter::S)
                {
                    xmas += 1;
                    // println!("S\n.A\n..M\n..X\n");
                }

                // ...X
                // ..M
                // .A
                // S
                if row_idx <= GRID_MAX
                    && col_idx >= 3
                    && grid[row_idx + 1][col_idx - 1] == Some(Letter::M)
                    && grid[row_idx + 2][col_idx - 2] == Some(Letter::A)
                    && grid[row_idx + 3][col_idx - 3] == Some(Letter::S)
                {
                    xmas += 1;
                }

                // ...S
                // ..A
                // .M
                // X
                if row_idx >= 3
                    && col_idx <= GRID_MAX
                    && grid[row_idx - 1][col_idx + 1] == Some(Letter::M)
                    && grid[row_idx - 2][col_idx + 2] == Some(Letter::A)
                    && grid[row_idx - 3][col_idx + 3] == Some(Letter::S)
                {
                    xmas += 1;
                }
            }
        }
    }

    Some(xmas)
}

pub fn part_two(input: &str) -> Option<u32> {
    let grid = parse(input);

    let mut x_mas = 0;

    for (row_idx, row) in grid.iter().enumerate() {
        for (col_idx, letter) in row.iter().enumerate() {
            if row_idx == 0 || row_idx == GRID_SIZE - 1 || col_idx == 0 || col_idx == GRID_SIZE - 1
            {
                continue;
            }

            if let Some(Letter::A) = letter {
                // p q
                //  a
                // r s
                let p = grid[row_idx - 1][col_idx - 1];
                let q = grid[row_idx - 1][col_idx + 1];
                let r = grid[row_idx + 1][col_idx - 1];
                let s = grid[row_idx + 1][col_idx + 1];

                let ps = p == Some(Letter::M) && s == Some(Letter::S)
                    || p == Some(Letter::S) && s == Some(Letter::M);

                let qr = q == Some(Letter::M) && r == Some(Letter::S)
                    || q == Some(Letter::S) && r == Some(Letter::M);

                if ps && qr {
                    x_mas += 1;
                }
            }
        }
    }

    Some(x_mas)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(18));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(9));
    }
}
