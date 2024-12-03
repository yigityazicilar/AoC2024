use advent_of_code_utils::read_file;

fn main() {
    let part1_solution = part1("input.txt");
    println!("Day 2 Part 1 Solution: {part1_solution}");
    
    let part2_solution = part2("input.txt");
    println!("Day 2 Part 2 Solution: {part2_solution}");
}

fn part1(path: &str) -> i64 {
    let lines = read_file(path);
    lines.iter()
        .map(|line| {
            line.split_whitespace()
                .map(|num| num.parse::<i64>().unwrap())
                .collect::<Vec<i64>>()
        })
        .filter(|nums| is_report_valid(nums)).count() as i64
    
}

fn part2(path: &str) -> i64 {
    let lines = read_file(path);
    lines.iter()
        .map(|line| {
            line.split_whitespace()
                .map(|num| num.parse::<i64>().unwrap())
                .collect::<Vec<i64>>()
        })
        .filter(|nums| {
            let valid_by_default = is_report_valid(nums);
            let valid_with_removal = nums.iter().enumerate().any(|(i, _)| {
                let mut nums_clone = nums.clone();
                nums_clone.remove(i);
                is_report_valid(&nums_clone)
            });
            valid_by_default || valid_with_removal       
        }).count() as i64
}

fn is_report_valid(nums: &Vec<i64>) -> bool {
    nums.windows(2).all(|w| {
        let abs_diff = (w[0] - w[1]).abs();
        abs_diff >= 1 && abs_diff <= 3
    }) && (nums.is_sorted() || nums.iter().rev().is_sorted())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1("example.txt"), 2);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2("example.txt"), 4);
    }
}