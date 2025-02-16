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
    // 判断输入数组是否为空
    if nums.is_empty() {
        return Vec::new();
    }

    // 对数组进行原地排序
    nums.sort_unstable();

    // 初始化用于保存重复元素结果的向量
    let mut duplicates = Vec::new();

    // 遍历排序后的数组，从第二个元素开始判断
    for i in 1..nums.len() {
        // 如果当前元素和前一个元素相等，说明存在重复
        if nums[i] == nums[i - 1] {
            // 检查是否为首次添加该重复元素
            if duplicates.is_empty() {
                duplicates.push(nums[i]);
            } else if *duplicates.last().unwrap() != nums[i] {
                duplicates.push(nums[i]);
            }
        }
    }

    // 返回重复元素集合
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
