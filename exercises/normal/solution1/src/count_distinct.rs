// src/count_distinct.rs
use std::collections::HashSet;

pub fn new_count_distinct(input_str: &str) -> usize {
    let elements: Vec<&str> = input_str.split(',').collect();
    let unique_elements: HashSet<&str> = elements.into_iter().collect();
    unique_elements.len()
}