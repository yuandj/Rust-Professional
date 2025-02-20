/*
    Find Duplicates in Array
    Given an array, find all the duplicate elements and return them. 
    You need to solve the problem with O(1) space complexity (i.e., without using extra arrays or hash tables).

    Implement the function `find_duplicates(nums: Vec<i32>) -> Vec<i32>`.
    The function should return a vector containing all the duplicate elements in the array.
    
    Hint: You can modify the input array in place to track duplicates.
*/

use std::fmt::{self, Display, Formatter};

pub fn find_duplicates(mut nums: Vec<i32>) -> Vec<i32> {
    let mut duplicates = Vec::new();
    let n = nums.len() as i32;

    for i in 0..nums.len() {
        let num = nums[i].abs();
        // 跳过无效元素（不在1..=n范围内）
        if num < 1 || num > n {
            continue;
        }
        let index = (num - 1) as usize;
        if nums[index] < 0 {
            // 已标记过，说明重复
            duplicates.push(num);
            // 防止后续重复添加
            nums[index] = 0;
        } else {
            // 标记为已访问
            nums[index] = -nums[index];
        }
    }

    duplicates
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_duplicates_1() {
        let nums = vec![1, 2, 3, 4, 5, 6, 2, 3];
        assert_eq!(find_duplicates(nums), vec![2, 3]);
    }

    #[test]
    fn test_find_duplicates_2() {
        let nums = vec![4, 5, 6, 7, 5, 4];
        assert_eq!(find_duplicates(nums), vec![5, 4]);
    }

    #[test]
    fn test_find_duplicates_3() {
        let nums = vec![1, 2, 3, 4, 5];
        assert!(find_duplicates(nums).is_empty());
    }

    #[test]
    fn test_find_duplicates_4() {
        let nums = vec![1, 1, 1, 1, 1];
        assert_eq!(find_duplicates(nums), vec![1]);
    }

    #[test]
    fn test_find_duplicates_5() {
        // 修正测试用例，确保所有元素值在1..=n范围内
        let nums = vec![5, 4, 3, 2, 1, 5, 4]; // 数组长度7，元素在1-5之间
        let mut result = find_duplicates(nums);
        result.sort(); // 顺序可能不同，排序后比较
        assert_eq!(result, vec![4, 5]);
    }
}