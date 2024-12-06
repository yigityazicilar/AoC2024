use std::collections::{BTreeSet, HashSet};

use advent_of_code_utils::read_file;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
} 

impl Direction {
    fn right_turn(&self) -> Self {
        match self {
            Self::Up => Self::Right,
            Self::Down => Self::Left,
            Self::Left => Self::Up,
            Self::Right => Self::Down,
        }
    }
    
    fn left_turn(&self) -> Self {
        match self {
            Self::Up => Self::Left,
            Self::Down => Self::Right,
            Self::Left => Self::Down,
            Self::Right => Self::Up,
        }
    }
}

#[derive(Debug)]
struct Input {
    row_obstacles: Vec<BTreeSet<usize>>,
    col_obstacles: Vec<BTreeSet<usize>>,
    start_pos: Position,
    num_rows: usize,
    num_cols: usize,
}

type Position = (usize, usize, Direction);

impl Input {
    fn new(path: &str) -> Self {
        let grid: Vec<Vec<char>> = read_file(path)
            .iter()
            .map(|line| line.chars().collect())
            .collect();
        
        let start_pos: Position = grid.iter().enumerate().find_map(|(r, row)| {
            row.iter().enumerate().find_map(|(c, &cell)| {
                match cell {
                    '^' => Some((r, c, Direction::Up)),
                    'v' => Some((r, c, Direction::Down)),
                    '<' => Some((r, c, Direction::Left)),
                    '>' => Some((r, c, Direction::Right)),
                    _ => None,
                }
            })
        }).unwrap(); 

        let num_rows = grid.len();
        let num_cols = grid[0].len();
        
        let mut row_obstacles: Vec<BTreeSet<usize>> = vec![BTreeSet::new(); num_rows];
        let mut col_obstacles: Vec<BTreeSet<usize>> = vec![BTreeSet::new(); num_cols];
        for r in 0..num_rows {
            for c in 0..num_cols {
                if grid[r][c] == '#' {
                    row_obstacles[r].insert(c);
                    col_obstacles[c].insert(r);
                }
            }
        }

        Self { row_obstacles, col_obstacles, start_pos, num_rows, num_cols }
    }
    
    fn insert_obstacle(&mut self, r: usize, c: usize) {
        self.row_obstacles[r].insert(c);
        self.col_obstacles[c].insert(r);
    }

    fn remove_obstacle(&mut self, r: usize, c: usize) {
        self.row_obstacles[r].remove(&c);
        self.col_obstacles[c].remove(&r);
    }
}

fn main() {
    let part1_solution = part1("input.txt");
    println!("Day 6 Part 1 Solution: {}", part1_solution);
    
    let part2_solution = part2("input.txt");
    println!("Day 6 Part 2 Solution: {}", part2_solution);
}

fn part1(path: &str) -> usize {
    let input= Input::new(path);

    let direction_changes = find_path(&input).expect("The default path is an infinite loop.");
    let path = get_path(input.start_pos, direction_changes, input.num_rows, input.num_cols); 
    
    path.len()
}

fn part2 (path: &str) -> usize {
    let mut input= Input::new(path);

    let direction_changes = find_path(&input).expect("The default path is an infinite loop.");
    let mut potential_obstacles = get_path(input.start_pos, direction_changes, input.num_rows, input.num_cols);
    potential_obstacles.retain(|(r, c)| *r != input.start_pos.0 || *c != input.start_pos.1); 

    let mut time_paradox_count = 0;
    for (r, c) in potential_obstacles {
        input.insert_obstacle(r, c);
        if let Err(_) = find_path(&input) {
            time_paradox_count += 1;
        }
        input.remove_obstacle(r, c);
    }

    time_paradox_count
}


fn find_path(input: &Input) -> Result<Vec<Position>, ()> {
    let Input { row_obstacles, col_obstacles, start_pos, .. } = input;
    let mut curr_pos: Position = *start_pos;
    let mut obstacles_hit: BTreeSet<Position> = BTreeSet::new();
    let mut direction_changes: Vec<Position> = vec![];
    while let Some((r, c, dir)) = next_obstacle(curr_pos, &row_obstacles, &col_obstacles) {
        if !obstacles_hit.insert((r, c, dir)) {
            return Err(());
        }
        match dir {
            Direction::Up => {
                curr_pos = (r + 1, c, dir.right_turn());
            }
            Direction::Down => {
                curr_pos = (r - 1, c, dir.right_turn());
            }
            Direction::Left => {
                curr_pos = (r, c + 1, dir.right_turn());
            }
            Direction::Right => {
                curr_pos = (r, c - 1, dir.right_turn());
            }
        }
        direction_changes.push(curr_pos);
    }
    
    Ok(direction_changes)
}

fn next_obstacle(curr_pos: Position, row_obstacles: &Vec<BTreeSet<usize>>, col_obstacles: &Vec<BTreeSet<usize>>) -> Option<Position> {
    let (r, c, dir) = curr_pos;
    match dir {
        Direction::Up => {
            col_obstacles[c].iter().rev().find(|&&row| row < r).map_or(None, |&row| Some((row, c, dir)))
        }
        Direction::Down => {
            col_obstacles[c].iter().find(|&&row| row > r).map_or(None, |&row| Some((row, c, dir)))
        }
        Direction::Left => {
            row_obstacles[r].iter().rev().find(|&&col| col < c).map_or(None, |&col| Some((r, col, dir)))
        }
        Direction::Right => {
            row_obstacles[r].iter().find(|&&col| col > c).map_or(None, |&col| Some((r, col, dir)))
        }
    }
}

fn get_path(start_pos: Position, direction_changes: Vec<Position>, num_rows: usize, num_cols: usize) -> HashSet<(usize, usize)> {
    let mut path = HashSet::new();
    let (mut start_row, mut start_col, mut start_dir) = start_pos;
    for (next_row, next_col, next_dir) in direction_changes {
        if start_row == next_row {
            for col in start_col.min(next_col)..=start_col.max(next_col) {
                path.insert((start_row, col));
            }
            start_col = next_col;
        } else if next_col == start_col {
            for row in start_row.min(next_row)..=start_row.max(next_row) {
                path.insert((row, start_col));
            }
            start_row = next_row;
        }
        start_dir = next_dir;
    }

    match start_dir {
        Direction::Up => {
            for row in 0..start_row {
                path.insert((row, start_col));
            }
        }
        Direction::Down => {
            for row in start_row..num_rows {
                path.insert((row, start_col));
            }
        }
        Direction::Left => {
            for col in 0..start_col {
                path.insert((start_row, col));
            }
        }
        Direction::Right => {
            for col in start_col..num_cols {
                path.insert((start_row, col));
            }
        }
    }

    path
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1("example.txt"), 41);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2("example.txt"), 6);
    }
}