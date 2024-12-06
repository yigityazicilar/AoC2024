use std::{collections::{HashMap, HashSet}, fs::read_to_string};

fn main() {
    let part1_solution = part1("input.txt");
    println!("Day 5 Part 1 Solution: {}", part1_solution);
    
    let part2_solution = part2("input.txt");
    println!("Day 5 Part 2 Solution: {}", part2_solution);
}

fn part1(path: &str) -> i64 {
    let content = read_to_string(path).unwrap().split("\n\n").map(|x| x.to_string()).collect::<Vec<String>>();
    let mut rules_map: HashMap<i64, HashSet<i64>> = HashMap::new();
    content[0].lines().for_each(|line| {
        let rule = line.split("|").map(|x| x.parse::<i64>().unwrap()).collect::<Vec<i64>>();
        if rules_map.contains_key(&rule[0]) {
            let before_set = rules_map.get_mut(&rule[0]).unwrap();
            before_set.insert(rule[1]);
        } else {
            let mut before_set = HashSet::new();
            before_set.insert(rule[1]);
            rules_map.insert(rule[0], before_set);
        }
    });

    let books = content[1].lines().map(|pages| {
        let page = pages.split(",").map(|x| x.parse::<i64>().unwrap()).collect::<Vec<i64>>();
        page
    }).collect::<Vec<Vec<i64>>>();
    
    let mid_values = books.iter().map(|pages| {
        let mut seen_pages: HashSet<i64> = HashSet::new();
        for page in pages {
            let before_pages_option = rules_map.get(page);
            if let Some(before_pages) = before_pages_option {
                if seen_pages.difference(before_pages).count() != seen_pages.len() {
                    return 0;
                }
            }
            seen_pages.insert(*page);
        }
        
        pages[pages.len().div_euclid(2)]
    }).collect::<Vec<i64>>();

    mid_values.into_iter().sum()
}

fn part2(path: &str) -> i64 {
    let content = read_to_string(path).unwrap().split("\n\n").map(|x| x.to_string()).collect::<Vec<String>>();
    let mut rules_map: HashMap<i64, HashSet<i64>> = HashMap::new();
    content[0].lines().for_each(|line| {
        let rule = line.split("|").map(|x| x.parse::<i64>().unwrap()).collect::<Vec<i64>>();
        if rules_map.contains_key(&rule[0]) {
            let before_set = rules_map.get_mut(&rule[0]).unwrap();
            before_set.insert(rule[1]);
        } else {
            let mut before_set = HashSet::new();
            before_set.insert(rule[1]);
            rules_map.insert(rule[0], before_set);
        }
    });

    let books = content[1].lines().map(|pages| {
        let page = pages.split(",").map(|x| x.parse::<i64>().unwrap()).collect::<Vec<i64>>();
        page
    }).collect::<Vec<Vec<i64>>>();

    let wrong_books: Vec<Vec<i64>> = books.into_iter().filter(|pages| {
        let mut seen_pages: HashSet<i64> = HashSet::new();
        for page in pages {
            let before_pages_option = rules_map.get(page);
            if let Some(before_pages) = before_pages_option {
                if seen_pages.difference(before_pages).count() != seen_pages.len() {
                    return true;
                }
            }
            seen_pages.insert(*page);
        }
        false
    }).collect();
    
    wrong_books.into_iter().map(|pages| {
        let mut correct_book = Vec::new();
        let pages_len = pages.len();
        let mut remaining_pages: HashSet<i64> = HashSet::from_iter(pages);
        for _ in 0..pages_len {
            let mut candidates: HashSet<i64> = HashSet::from_iter(remaining_pages.clone());
            for page in &remaining_pages {
                let before_pages_option = rules_map.get(page);
                if let Some(before_pages) = before_pages_option {
                    candidates = candidates.difference(before_pages).map(|x| *x).collect();
                }
            }
            assert!(candidates.len() == 1);
            let next_page = candidates.into_iter().next().unwrap();
            remaining_pages.remove(&next_page);
            correct_book.push(next_page);
        }
        correct_book[correct_book.len().div_euclid(2)]
    }).sum()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1("example.txt"), 143);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2("example.txt"), 123);
    }
}