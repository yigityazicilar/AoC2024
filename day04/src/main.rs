use advent_of_code_utils::read_file;

fn main() {
    let part1_solution = part1("input.txt");
    println!("Day 4 Part 1 Solution: {part1_solution}");
    
    let part2_solution = part2("input.txt");
    println!("Day 4 Part 2 Solution: {part2_solution}"); 
}

fn part1(path: &str) -> i64 {
    let grid: Vec<Vec<char>> = read_file(path)
        .iter()
        .map(|line| line.chars().collect())
        .collect();
    let mut count = 0;
    let (h, w) = (grid.len() as i64, grid[0].len() as i64);
    let dirs = [(-1,-1),(-1,0),(-1,1),(0,-1),(0,1),(1,-1),(1,0),(1,1)];

    for r in 0..h {
        for c in 0..w {
            if grid[r as usize][c as usize] != 'X' {
                continue;
            }
            for (dr, dc) in dirs.iter() {
                if r + dr * 3 >= h || r + dr * 3 < 0 || c + dc * 3 >= w || c + dc * 3 < 0 {
                    continue; 
                }
                
                let word = (0..4).map(|i| grid[(r + dr * i) as usize][(c + dc * i) as usize]).collect::<String>();
                if word == "XMAS" {
                    count += 1;
                }
            }
        }
    }
    count
}

fn part2(path: &str) -> i64 {
    let grid: Vec<Vec<char>> = read_file(path)
        .iter()
        .map(|line| line.chars().collect())
        .collect();

    let mut count = 0;
    let (h, w) = (grid.len() as i64, grid[0].len() as i64);

    for r in 0..h {
        for c in 0..w {
            if grid[r as usize][c as usize] != 'A' {
                continue;
            }
            if r - 1 < 0 || r + 1 >= h || c - 1 < 0 || c + 1 >= w {
                continue;
            } 
            
            let d1 = (-1..=1).map(|i| grid[(r + i) as usize][(c + i) as usize]).collect::<String>();
            let d2 = (-1..=1).map(|i| grid[(r + i) as usize][(c - i) as usize]).collect::<String>();
            
            if (d1 == "SAM" || d1 == "MAS") && (d2 == "SAM" || d2 == "MAS") {
                count += 1;
            }
        }
    }
    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1("example.txt"), 18);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2("example.txt"), 9);
    }
}