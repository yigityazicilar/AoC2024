use std::{collections::{HashMap, HashSet}, fs::read_to_string};

fn main() {
    let part1_solution = part1("input.txt");
    println!("Day 5 Part 1 Solution: {}", part1_solution);
    
    let part2_solution = part2("input.txt");
    println!("Day 5 Part 2 Solution: {}", part2_solution);
}

fn part1(path: &str) -> i64 {
    let content = read_to_string(path).unwrap().split("\n\n").map(|x| x.to_string()).collect::<Vec<String>>();
    let rules_map: HashMap<i64, HashSet<i64>> = get_rules_map(&content[0]);
    let books = get_books(&content[1]);
    
    let (correct_books, _) = classify_books(&rules_map, &books);
    
    correct_books.iter().map(|book| {
        book[book.len().div_euclid(2)]
    }).sum()
}

fn part2(path: &str) -> i64 {
    let content = read_to_string(path).unwrap().split("\n\n").map(|x| x.to_string()).collect::<Vec<String>>();
    let rules_map: HashMap<i64, HashSet<i64>> = get_rules_map(&content[0]);
    let books = get_books(&content[1]);

    let (_, wrong_books) = classify_books(&rules_map, &books);
    
    let corrected_books = wrong_books.iter().map(|book| correct_book(book, &rules_map));
    
    corrected_books.map(|pages| {
        pages[pages.len().div_euclid(2)]
    }).sum()
}

fn get_rules_map(content: &String) -> HashMap<i64, HashSet<i64>> {
    let mut rules_map: HashMap<i64, HashSet<i64>> = HashMap::new();
    content.lines().for_each(|line| {
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

    rules_map
}

fn get_books(content: &String) -> Vec<Vec<i64>> {
    content.lines().map(|book| {
        let page = book.split(",")
            .map(|x| x.parse::<i64>().unwrap())
            .collect::<Vec<i64>>();
        page
    }).collect()
}

fn classify_books(rules_map: &HashMap<i64, HashSet<i64>>, books: &Vec<Vec<i64>>) -> (Vec<Vec<i64>>, Vec<Vec<i64>>) {
    let mut correct_books = Vec::new();
    let mut wrong_books = Vec::new();
    'books: for book in books {
        let mut seen_pages: HashSet<i64> = HashSet::new();
        for page in book {
            let before_pages_option = rules_map.get(page);
            if let Some(before_pages) = before_pages_option {
                if seen_pages.intersection(before_pages).count() > 0 {
                    wrong_books.push(book.clone());
                    continue 'books;
                }
            }
            seen_pages.insert(*page);
        }
        correct_books.push(book.clone());
    }

    (correct_books, wrong_books)
}

fn correct_book(book: &Vec<i64>, rules_map: &HashMap<i64, HashSet<i64>>) -> Vec<i64> {
    let mut corrected_book = Vec::new();
    let mut remaining_pages: HashSet<i64> = HashSet::from_iter(book.clone());
    for _ in 0..(book.len()) {
        let mut candidate_pages: HashSet<i64> = HashSet::from_iter(remaining_pages.clone());
        for page in &remaining_pages {
            let before_pages_option = rules_map.get(page);
            if let Some(before_pages) = before_pages_option {
                candidate_pages.retain(|page| !before_pages.contains(page));
            }
        }
        assert!(candidate_pages.len() == 1); // Check if there is only one candidate page
        let next_page = candidate_pages.into_iter().next().unwrap();
        remaining_pages.remove(&next_page);
        corrected_book.push(next_page);
    }

    corrected_book
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