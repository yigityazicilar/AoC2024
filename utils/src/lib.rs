use std::fs::read_to_string;
use std::vec::Vec;


pub fn read_file(path: &str) -> Vec<String> {
    read_to_string(path)
        .expect("Failed to read file")
        .lines()
        .map(|line| line.to_string())
        .collect()
        
}