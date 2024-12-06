use std::collections::HashSet;

use advent_of_code_utils::read_file;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
} 

fn main() {
    let part1_solution = part1("input.txt");
    println!("Day 6 Part 1 Solution: {}", part1_solution.len());
    
    let part2_solution = part2("input.txt");
    println!("Day 6 Part 2 Solution: {}", part2_solution);
}

fn part1(path: &str) -> HashSet<(usize, usize)> {
    let grid: Vec<Vec<char>> = read_file(path)
        .iter()
        .map(|line| line.chars().collect())
        .collect();

    let start_pos: (usize, usize, Direction) = grid.iter().enumerate().find_map(|(r, row)| {
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
    
    let mut path: Vec<(usize, usize, Direction)> = vec![start_pos];
    loop {
        let (r,c, dir) = path.last().unwrap();
        match dir {
            Direction::Up => {
                if *r == 0 {
                    break;
                }
                if grid[r-1][*c] == '#' {
                    path.push((*r, *c, Direction::Right))
                } else {
                    path.push((r-1, *c, Direction::Up))
                }
            }
            Direction::Down => {
                if *r == grid.len() - 1 {
                    break;
                }
                if grid[r+1][*c] == '#' {
                    path.push((*r, *c, Direction::Left))
                } else {
                    path.push((r+1, *c, Direction::Down))
                }
            }
            Direction::Left => {
                if *c == 0 {
                    break;
                }
                if grid[*r][c-1] == '#' {
                    path.push((*r, *c, Direction::Up))
                } else {
                    path.push((*r, c-1, Direction::Left))
                }
            }
            Direction::Right => {
                if *c == grid[0].len() - 1 {
                    break;
                }
                if grid[*r][c+1] == '#' {
                    path.push((*r, *c, Direction::Down))
                } else {
                    path.push((*r, c+1, Direction::Right))
                }
            }
        }
    }
    let distinct_positions = path.into_iter().map(|(r, c, _)| (r, c)).collect::<HashSet<(usize, usize)>>();
    distinct_positions
}

fn part2(path: &str) -> i64 {
    let mut grid: Vec<Vec<char>> = read_file(path)
        .iter()
        .map(|line| line.chars().collect())
        .collect();

    let start_pos: (usize, usize, Direction) = grid.iter().enumerate().find_map(|(r, row)| {
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

    let mut initial_path = part1(path);
    initial_path.remove(&(start_pos.0, start_pos.1));
    
    let mut time_paradox_count: i64 = 0;
    for (obs_r, obs_c) in initial_path {
        grid[obs_r][obs_c] = '#';
        let mut seen_pos: HashSet<(usize, usize, Direction)> = HashSet::from_iter(vec![start_pos]);
        let mut path: Vec<(usize, usize, Direction)> = vec![start_pos];
        loop {
            let (r,c, dir) = path.last().unwrap();
            match dir {
                Direction::Up => {
                    if *r == 0 {
                        break;
                    }
                    let new_pos;
                    if grid[r-1][*c] == '#' {
                        new_pos = (*r, *c, Direction::Right);
                    } else {
                        new_pos = (r-1, *c, Direction::Up);
                    }
                    if seen_pos.contains(&new_pos) {
                        time_paradox_count += 1;
                        break;
                    }
                    seen_pos.insert(new_pos);
                    path.push(new_pos);
                }
                Direction::Down => {
                    if *r == grid.len() - 1 {
                        break;
                    }
                    let new_pos;
                    if grid[r+1][*c] == '#' {
                        new_pos = (*r, *c, Direction::Left);
                    } else {
                        new_pos = (r+1, *c, Direction::Down);
                    }
                    if seen_pos.contains(&new_pos) {
                        time_paradox_count += 1;
                        break;
                    }
                    seen_pos.insert(new_pos);
                    path.push(new_pos);
                }
                Direction::Left => {
                    if *c == 0 {
                        break;
                    }
                    let new_pos;
                    if grid[*r][c-1] == '#' {
                        new_pos = (*r, *c, Direction::Up);
                    } else {
                        new_pos = (*r, c-1, Direction::Left);
                    }
                    if seen_pos.contains(&new_pos) {
                        time_paradox_count += 1;
                        break;
                    }
                    seen_pos.insert(new_pos);
                    path.push(new_pos);
                }
                Direction::Right => {
                    if *c == grid[0].len() - 1 {
                        break;
                    }
                    let new_pos;
                    if grid[*r][c+1] == '#' {
                        new_pos = (*r, *c, Direction::Down);
                    } else {
                        new_pos = (*r, c+1, Direction::Right);
                    }
                    if seen_pos.contains(&new_pos) {
                        time_paradox_count += 1;
                        break;
                    }
                    seen_pos.insert(new_pos);
                    path.push(new_pos);
                }
            }
        }
        
        grid[obs_r][obs_c] = '.';
    }
    time_paradox_count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1("example.txt").len(), 41);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2("example.txt"), 6);
    }
}