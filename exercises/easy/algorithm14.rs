/*
    Find Duplicates in Array
    Given an array, find all the duplicate elements and return them.
    You need to solve the problem with O(1) space complexity (i.e., without using extra arrays or hash tables).

    Implement the function `find_duplicates(nums: Vec<i32>) -> Vec<i32>`.
    The function should return a vector containing all the duplicate elements in the array.

    Hint: You can modify the input array in place to track duplicates.
*/
/* 中文:
    在数组中查找重复元素
    给定一个数组，找到所有重复的元素并返回它们。
    你需要使用 O(1) 空间复杂度解决这个问题（即，不使用额外的数组或哈希表）。

    实现函数 `find_duplicates(nums: Vec<i32>) -> Vec<i32>`。
    该函数应返回一个向量，包含数组中所有重复的元素。

    提示：你可以就地修改输入数组来跟踪重复元素。
*/

use std::fmt::{self, Display, Formatter};

pub fn find_duplicates(nums: Vec<i32>) -> Vec<i32> {
    // TODO: Implement the logic to find all duplicates in the array
    // 中文: 实现查找所有重复元素的逻辑
    Vec::new() // Placeholder return value
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_duplicates_1() {
        let nums = vec![1, 2, 3, 4, 5, 6, 2, 3];
        let result = find_duplicates(nums);
        println!("Duplicates: {:?}", result);
        assert_eq!(result, vec![2, 3]);
    }

    #[test]
    fn test_find_duplicates_2() {
        let nums = vec![4, 5, 6, 7, 5, 4];
        let result = find_duplicates(nums);
        println!("Duplicates: {:?}", result);
        assert_eq!(result, vec![4, 5]);
    }

    #[test]
    fn test_find_duplicates_3() {
        let nums = vec![1, 2, 3, 4, 5];
        let result = find_duplicates(nums);
        println!("Duplicates: {:?}", result);
        assert_eq!(result, Vec::<i32>::new());
    }

    #[test]
    fn test_find_duplicates_4() {
        let nums = vec![1, 1, 1, 1, 1];
        let result = find_duplicates(nums);
        println!("Duplicates: {:?}", result);
        assert_eq!(result, vec![1]);
    }

    #[test]
    fn test_find_duplicates_5() {
        let nums = vec![10, 9, 8, 7, 6, 7, 8];
        let result = find_duplicates(nums);
        println!("Duplicates: {:?}", result);
        assert_eq!(result, vec![7, 8]);
    }
}
