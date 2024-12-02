use advent_of_code_utils::read_file;

fn main() {
    let part1_solution = part1("input.txt");
    println!("Day 1 Part 1 Solution: {part1_solution}");
    
    let part2_solution = part2("input.txt");
    println!("Day 1 Part 2 Solution: {part2_solution}");
    
}

fn part1(path: &str) -> i64 {
    let lines = read_file(path);
    let (mut left_list, mut right_list): (Vec<i64>, Vec<i64>) = 
        lines.iter()
            .map(|line| {
                let nums = line.split_whitespace()
                    .map(|num| num.parse::<i64>().unwrap())
                    .collect::<Vec<i64>>();

                (nums[0], nums[1])
            })
            .unzip();

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
    let (left_list, right_list): (Vec<i64>, Vec<i64>) = 
        lines.into_iter()
            .map(|line| {
                let nums = line.split_whitespace()
                    .map(|num| num.parse::<i64>().unwrap())
                    .collect::<Vec<i64>>();

                (nums[0], nums[1])
            })
            .unzip();

    left_list.iter()
        .map(|num| {
            let count: i64 = right_list.iter().filter(|&right| num == right).count() as i64;
            num * count
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
