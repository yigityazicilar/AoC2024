use std::collections::HashMap;

use advent_of_code_utils::read_file;

fn main() {
    let part1_solution = part1("input.txt");
    println!("Day 1 Part 1 Solution: {part1_solution}");
    
    let part2_solution = part2("input.txt");
    println!("Day 1 Part 2 Solution: {part2_solution}");
    
}

fn part1(path: &str) -> i64 {
    let lines = read_file(path);
    let mut left_list: Vec<i64> = Vec::new();
    let mut right_list: Vec<i64> = Vec::new();
    
    lines
        .iter()
        .for_each(|line| {
            let nums = line
                        .split_whitespace()
                        .map(|num| num.parse::<i64>().unwrap())
                        .collect::<Vec<i64>>();
            left_list.push(nums[0]);
            right_list.push(nums[1]);
        });
    
    left_list.sort();
    right_list.sort();

    left_list
        .iter()
        .zip(right_list.iter())
        .map(|(left, right)| (left - right).abs())
        .sum()
}

fn part2(path: &str) -> i64 {
    let lines = read_file(path);
    let mut left_list: Vec<i64> = Vec::new();
    let mut count_map: HashMap<i64, i64> = HashMap::new();
    
    lines
        .iter()
        .for_each(|line| {
            let nums = line
                        .split_whitespace()
                        .map(|num| num.parse::<i64>().unwrap())
                        .collect::<Vec<i64>>();
            left_list.push(nums[0]);
            if let Some(count) = count_map.get_mut(&nums[1]) {
                *count += 1;
            } else {
                count_map.insert(nums[1], 1);
            }
        });

    left_list
        .into_iter()
        .map(|num| {
            num * count_map.get(&num).unwrap_or(&0)
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let path = "example.txt";
        assert_eq!(part1(path), 11);
    }

    #[test]
    fn test_part2() {
        let path = "example.txt";
        assert_eq!(part2(path), 31);
    }
}
