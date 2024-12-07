use advent_of_code_utils::read_file;

#[derive(Debug)]
struct Calibration {
    goal: usize,
    numbers: Vec<usize>,
}

fn main() {
    let part1_solution = part1("input.txt");
    println!("Day 7 Part 1 Solution: {part1_solution}");
    
    let part2_solution = part2("input.txt");
    println!("Day 7 Part 2 Solution: {part2_solution}");
}

fn parse_calibration(line: String) -> Calibration {
    let split_line = line.split(": ").collect::<Vec<&str>>();
    let goal = split_line[0].parse::<usize>().unwrap();
    let numbers = split_line[1].split(" ").map(|x| x.parse::<usize>().unwrap()).collect();
    return Calibration { goal, numbers };
}

fn is_possible(calibration: Calibration, part2: bool) -> bool {
    let Calibration { goal, mut numbers } = calibration;
    if let Some(last_num) = numbers.pop() {
        if last_num == goal {
            return true;
        }

        if part2 {
            let last_num_len = last_num.to_string().len();
            let (div, rem) = (goal / 10usize.pow(last_num_len as u32), goal % 10usize.pow(last_num_len as u32));
            
            if rem == last_num && is_possible(Calibration { goal: div, numbers: numbers.clone() }, part2) {
                return true;
            }
        }

        if goal % last_num == 0 && is_possible(Calibration { goal: goal / last_num, numbers: numbers.clone() }, part2) {
            return true;
        }

        if goal >= last_num && is_possible(Calibration { goal: goal - last_num, numbers: numbers.clone() }, part2) {
            return true;
        }
    }
    
    return false;
}

fn part1(path: &str) -> usize {
    let lines = read_file(path);
    let calibrations = lines.iter().map(|line| parse_calibration(line.to_string())).collect::<Vec<Calibration>>();
    calibrations.into_iter().map(|calibration| {
        let goal = calibration.goal;
        if is_possible(calibration, false) {
            goal
        } else {
            0
        }
    }).sum::<usize>()
}

fn part2(path: &str) -> usize {
    let lines = read_file(path);
    let calibrations = lines.iter().map(|line| parse_calibration(line.to_string())).collect::<Vec<Calibration>>();
    calibrations.into_iter().map(|calibration| {
        let goal = calibration.goal;
        if is_possible(calibration, true) {
            goal
        } else {
            0
        }
    }).sum::<usize>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1("example.txt"), 3749);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2("example.txt"), 11387);
    }
}