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

pub fn find_duplicates(mut nums: Vec<i32>) -> Vec<i32> {
    // 如果传入的数组为空，直接返回一个空的向量
    if nums.is_empty() {
        return Vec::new();
    }

    // 对数组进行原地排序，这样相同的元素会挨在一起
    nums.sort_unstable();

    // 初始化一个向量，用于存储所有找到的重复元素
    let mut duplicates = Vec::new();

    // 遍历排序后数组中所有长度为2的窗口
    for pair in nums.windows(2) {
        // 检查窗口中的两个元素是否相等
        let equal = pair[0] == pair[1];
        // 判断当前的重复数字是否还未被加入过duplicates中
        let is_new_duplicate = duplicates.last().map_or(true, |&last| last != pair[0]);
        // 如果两个元素相等且当前重复数字尚未加入到duplicates中，则添加该数字
        if equal && is_new_duplicate {
            duplicates.push(pair[0]);
        }
    }

    // 返回包含所有重复元素的向量
    duplicates
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
