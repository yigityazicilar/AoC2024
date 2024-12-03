use advent_of_code_utils::read_file;
use regex::Regex;


fn main() {
    let part1_solution = part1("input.txt");
    println!("Day 3 Part 1 Solution: {part1_solution}");
    
    let part2_solution = part2("input.txt");
    println!("Day 3 Part 2 Solution: {part2_solution}");
}

fn part1(path: &str) -> i64 {
    let lines = read_file(path);
    let program = lines.join("");
    let regex = Regex::new(r"mul\((?<a>\d{1,3}),(?<b>\d{1,3})\)").unwrap();
    regex.captures_iter(&program).map(|caps| {
        let lhs = caps.name("a").unwrap().as_str().parse::<i64>().unwrap();
        let rhs = caps.name("b").unwrap().as_str().parse::<i64>().unwrap();
        lhs * rhs
    }).sum::<i64>()

}

fn part2(path: &str) -> i64 {
    let lines = read_file(path);
    let program = lines.join("");
    let regex = Regex::new(r"mul\((?<a>\d{1,3}),(?<b>\d{1,3})\)|do\(\)|don't\(\)").unwrap();
    let (val, _) = regex.captures_iter(&program).fold((0, true), |acc, caps| {
        if let Some(lhs) = caps.name("a") {
            if acc.1 {
                let rhs = caps.name("b").unwrap().as_str().parse::<i64>().unwrap();
                let lhs = lhs.as_str().parse::<i64>().unwrap();
                (acc.0 + lhs * rhs, true)
            } else {
                acc
            }
        } else {
            let capture_str = caps.get(0).unwrap().as_str();
            if  capture_str == "do()" {
                (acc.0, true)
            } else {
                (acc.0, false)
            }
        }
    });
    val
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1("example.txt"), 161);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2("example.txt"), 48);
    }
}