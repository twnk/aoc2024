use std::collections::BTreeSet;

advent_of_code::solution!(6);

#[cfg(test)]
const GRID_SIZE: usize = 10;

#[cfg(not(test))]
const GRID_SIZE: usize = 130;

type Grid = [[u8; GRID_SIZE]; GRID_SIZE];
type Path = [[bool; GRID_SIZE]; GRID_SIZE];

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
struct Pos {
    row: usize,
    col: usize,
}

const DOT: u8 = b'.';
const OBSTACLE: u8 = b'#';
const GUARD: u8 = b'^';

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

#[derive(Clone, Copy)]
enum Move {
    Position, // (Pos),
    Direction(Direction)
}

fn sum_res(path: [[bool; GRID_SIZE]; GRID_SIZE]) -> u32 {
    let mut sum = 0;
    for row in path {
        for x in row {
            if x {
                sum += 1
            }
        }
    }

    sum
}

fn parse_input(input: &str) -> (Grid, Pos) {
    let mut grid = [[0 as u8; GRID_SIZE]; GRID_SIZE];
    
    let mut guard_row: usize = 0;
    let mut guard_col: usize = 0;

    for (row_idx, row) in input.as_bytes().chunks_exact(GRID_SIZE + 1).enumerate() {
        grid[row_idx][..GRID_SIZE].copy_from_slice(&row[..GRID_SIZE]);

        if let Some(p) = row.iter().position(|c| c == &GUARD) {
            guard_row = row_idx;
            guard_col = p;
            grid[guard_row][guard_col] = DOT;
            
        }
    }

    (
        grid,

        Pos {
            row: guard_row,
            col: guard_col,
        },
    )
}

fn next_position(guard: &Pos, guard_direction: &Direction) -> Option<Pos> {
    match guard_direction {
        Direction::Up => {
            if guard.row == 0 {
                None
            } else {
                Some(Pos {
                    row: guard.row - 1,
                    col: guard.col,
                })
            }
        }
        Direction::Right => {
            if guard.col == GRID_SIZE - 1 {
                None
            } else {
                Some(Pos {
                    row: guard.row,
                    col: guard.col + 1,
                })
            }
        }
        Direction::Down => {
            if guard.row == GRID_SIZE - 1 {
                None
            } else {
                Some(Pos {
                    row: guard.row + 1,
                    col: guard.col,
                })
            }
        }
        Direction::Left => {
            if guard.col == 0 {
                None
            } else {
                Some(Pos {
                    row: guard.row,
                    col: guard.col - 1,
                })
            }
        }
    }
}

fn rotate_guard(dir: &Direction) -> Direction {
    match dir {
        Direction::Up => Direction::Right,
        Direction::Right => Direction::Down,
        Direction::Down => Direction::Left,
        Direction::Left => Direction::Up,
    }
}

fn next_move(grid: &Grid, next_pos: &Pos, guard_direction: &Direction) -> Move {
    match grid[next_pos.row][next_pos.col] {
        DOT => {
            Move::Position
        }
        OBSTACLE => {
            let next_dir = rotate_guard(guard_direction);

            Move::Direction(next_dir)
        }
        _ => {
            unreachable!()
        }
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let (grid, mut guard_pos) = parse_input(input);
    let mut path: Path = [[false; GRID_SIZE]; GRID_SIZE];
    path[guard_pos.row][guard_pos.col] = true;
    let mut guard_direction = Direction::Up;

    loop {
        let next_pos = if let Some(pos) = next_position(&guard_pos, &guard_direction) {
            pos
        } else {
            return Some(sum_res(path))
        };

        match next_move(&grid, &next_pos, &guard_direction) {
            Move::Position => {
                path[next_pos.row][next_pos.col] = true;
                guard_pos = next_pos;
            },
            Move::Direction(direction) => {
                guard_direction = direction;
            },
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Step {
    pos: Pos,
    dir: Direction
}

pub fn part_two(input: &str) -> Option<u32> {
    let (mut grid, mut guard_pos) = parse_input(input);
    let start_pos = guard_pos.clone();
    let mut guard_direction = Direction::Up;

    let mut turns: BTreeSet<Step> = BTreeSet::new();
    let mut obstacle_turns: BTreeSet<Step> = BTreeSet::new();

    let mut obstacles: BTreeSet<Pos> = BTreeSet::new();
    
    loop {
        let next_pos = if let Some(pos) = next_position(&guard_pos, &guard_direction) {
            pos
        } else {
            #[cfg(test)]
            println!("obstacles: {:?}", obstacles);
            println!("start pos obstacle?: {}", obstacles.contains(&start_pos));
            return Some(obstacles.len() as u32)
        };

        match next_move(&grid, &next_pos, &guard_direction) {
            Move::Position => {
                // insert an obstacle where the next position would be, then execute the loop until we
                // either retrace a step or walk off the map. then remove the obstacle and proceed
                let obstacle_pos = next_pos;

                // create obstacle - but we know the guard now has to rotate
                grid[obstacle_pos.row][obstacle_pos.col] = OBSTACLE;
                // let mut obstacle_guard_dir = rotate_guard(&guard_direction);
                // let mut obstacle_guard_pos = guard_pos;

                obstacle_turns.clear();
                // obstacle_turns.insert(Step { pos: obstacle_guard_pos, dir: obstacle_guard_dir });

                let mut obstacle_guard_dir = Direction::Up;
                let mut obstacle_guard_pos = start_pos;

                'prospecting: loop {
                    let obstacle_guard_next_pos = if let Some(p) = next_position(&obstacle_guard_pos, &obstacle_guard_dir) {
                        p
                    } else { break 'prospecting; };

                    match next_move(&grid, &obstacle_guard_next_pos, &obstacle_guard_dir) {
                        Move::Position => {
                            obstacle_guard_pos = obstacle_guard_next_pos;
                        },
                        Move::Direction(direction) => {
                            obstacle_guard_dir = direction;
                             // next position is an obstacle, so position is retained
                            let step = Step { pos: obstacle_guard_pos, dir: direction };
                            
                            if obstacle_turns.contains(&step) {
                                obstacles.insert(obstacle_pos);
                                break 'prospecting;
                            }

                            obstacle_turns.insert(step);
                        },
                    }

                }

                grid[obstacle_pos.row][obstacle_pos.col] = DOT;

                guard_pos = next_pos;
            },
            Move::Direction(direction) => {
                guard_direction = direction;
                // next position is an obstacle, so position is retained
                turns.insert(Step {pos: guard_pos, dir: direction});
            },
        }
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(41));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }
}
