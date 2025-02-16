/*
    Find Missing Number in Array
    Given an array containing `n-1` numbers in the range from `1` to `n`, find the missing number.
    The array is not sorted, and each number in the range appears exactly once except one.
    You need to solve this problem in O(n) time complexity and O(1) space complexity.
    Implement the function `find_missing_number(nums: Vec<i32>) -> i32`.
    The function should return the missing number.

    You are required to find an optimal solution with O(n) time complexity and O(1) space complexity.

    Hint: Use the sum of the first `n` numbers and subtract the sum of the array elements to find the missing number.
*/

/* 中文翻译
    查找数组中缺失的数字
    给定一个数组，其中包含范围从 1 到 n 的 n-1 个数字，找出缺失的数字。
    数组未排序，并且除了一个数字以外，范围内的每个数字均出现一次。
    需要以 O(n) 的时间复杂度和 O(1) 的空间复杂度解决这个问题。
    实现函数 find_missing_number(nums: Vec<i32>) -> i32。
    函数应返回缺失的数字。

    要求找到时间复杂度为 O(n) 且空间复杂度为 O(1) 的最优解决方案。

    提示：使用前 n 个数字的和减去数组元素的和来找出缺失的数字。
*/

use std::fmt::{self, Display, Formatter};

pub fn find_missing_number(nums: Vec<i32>) -> i32 {
    // 计算实际应该有的数字个数
    let n = (nums.len() + 1) as i32;

    // 计算从 1 到 n 的总和
    let expected = n * (n + 1) / 2;

    // 遍历数组并计算当前数字的和
    let actual: i32 = nums.iter().sum();

    // 返回缺失的数字，即总和与数组中数字和之差
    expected - actual
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_missing_number_1() {
        let nums = vec![3, 7, 1, 2, 8, 4, 5];
        let missing = find_missing_number(nums);
        println!("Missing number is {}", missing);
        assert_eq!(missing, 6);
    }

    #[test]
    fn test_missing_number_2() {
        let nums = vec![1, 2, 4, 5];
        let missing = find_missing_number(nums);
        println!("Missing number is {}", missing);
        assert_eq!(missing, 3);
    }

    #[test]
    fn test_missing_number_3() {
        let nums = vec![2, 3, 4, 5, 6, 7, 8, 9];
        let missing = find_missing_number(nums);
        println!("Missing number is {}", missing);
        assert_eq!(missing, 1);
    }

    #[test]
    fn test_missing_number_4() {
        let nums = vec![1, 2, 3, 5, 6];
        let missing = find_missing_number(nums);
        println!("Missing number is {}", missing);
        assert_eq!(missing, 4);
    }
}
