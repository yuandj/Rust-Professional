use std::collections::HashSet;

pub fn new_count_distinct(input_str: &str) -> usize {
    // 将输入字符串按逗号分隔成多个元素
    let elements: Vec<&str> = input_str.split(',').collect();
    
    // 使用 HashSet 来存储不重复的元素
    let unique_elements: HashSet<&str> = elements.into_iter().collect();
    
    // 返回 HashSet 的大小
    unique_elements.len()
}
