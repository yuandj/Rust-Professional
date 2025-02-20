/*
    Anagram Check
    Given two strings, check if they are anagrams of each other. 
    Anagrams are words or phrases formed by rearranging the letters of another, 
    using all the original letters exactly once. 
    The strings may contain spaces or punctuation, but you need to ignore them while checking.

    You need to implement the function `are_anagrams(s1: String, s2: String) -> bool`.
    The function should return `true` if the two strings are anagrams, and `false` otherwise.

    Hint: Consider normalizing the strings by removing non-alphabetical characters and converting to lowercase before checking.
*/

use std::fmt::{self, Display, Formatter};

pub fn are_anagrams(s1: String, s2: String) -> bool {
    // 预处理函数：转换为小写，过滤非字母字符，并收集为字符向量
    let normalize = |s: String| -> Vec<char> {
        s.to_lowercase()
            .chars()
            .filter(|c| c.is_ascii_alphabetic())
            .collect()
    };

    // 对两个字符串进行预处理
    let mut s1_chars = normalize(s1);
    let mut s2_chars = normalize(s2);

    // 排序字符向量
    s1_chars.sort();
    s2_chars.sort();

    // 比较排序后的字符向量是否相同
    s1_chars == s2_chars
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_anagram_1() {
        let s1 = "listen".to_string();
        let s2 = "silent".to_string();
        assert!(are_anagrams(s1, s2));
    }

    #[test]
    fn test_anagram_2() {
        let s1 = "evil".to_string();
        let s2 = "vile".to_string();
        assert!(are_anagrams(s1, s2));
    }

    #[test]
    fn test_anagram_3() {
        let s1 = "hello".to_string();
        let s2 = "world".to_string();
        assert!(!are_anagrams(s1, s2));
    }

    #[test]
    fn test_anagram_4() {
        let s1 = "Clint Eastwood".to_string();
        let s2 = "Old West Action".to_string();
        assert!(are_anagrams(s1, s2));
    }

    #[test]
    fn test_anagram_5() {
        let s1 = "Astronomer".to_string();
        let s2 = "Moon starer".to_string();
        assert!(are_anagrams(s1, s2));
    }
}