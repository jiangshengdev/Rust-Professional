/*
    Find Intersection of Two Arrays
    Given two arrays, find the intersection of the arrays and return the elements of the intersection (without duplicates).
    The result should not contain any duplicate elements.

    You need to implement the function `intersection(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32>`.
    The function should return a vector containing all the elements that are in both arrays.

    Hint: You can solve this problem using sorting, hash sets, or the two-pointer technique.
*/
/*
    求两个数组的交集
    给定两个数组，找出两个数组的交集并返回交集中的元素（不包含重复元素）。
    结果不应包含任何重复元素。

    你需要实现函数 `intersection(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32>`。
    该函数应该返回一个包含两个数组中共同元素的向量。

    提示：你可以使用排序、哈希集合或者双指针技术来解决这个问题。
*/

use std::fmt::{self, Display, Formatter};

pub fn intersection(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
    // 复制原数组用于排序，避免修改原始向量
    let mut sorted1 = nums1;
    let mut sorted2 = nums2;

    // 对第一个数组进行排序
    sorted1.sort();

    // 对第二个数组进行排序
    sorted2.sort();

    // 初始化双指针，分别指向两个数组的起始位置
    let mut i = 0;
    let mut j = 0;

    // 用于存放交集结果
    let mut result = Vec::new();

    // 当两个指针都未超出各自数组时，进入比较循环
    while i < sorted1.len() && j < sorted2.len() {
        // 如果两个指针指向的值相等
        if sorted1[i] == sorted2[j] {
            // 如果结果为空或当前值与上一次添加的值不相等
            if result.is_empty() {
                // 添加到结果中
                result.push(sorted1[i]);
            } else {
                // 检查是否与结果中最后一个元素不同
                if *result.last().unwrap() != sorted1[i] {
                    result.push(sorted1[i]);
                }
            }

            // 同时将两个指针后移
            i += 1;
            j += 1;
        }
        // 如果第一个数组中的值较小，则移动第一个数组的指针
        else if sorted1[i] < sorted2[j] {
            i += 1;
        }
        // 否则移动第二个数组的指针
        else {
            j += 1;
        }
    }

    // 返回最终的交集结果
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_intersection_1() {
        let nums1 = vec![1, 2, 2, 1];
        let nums2 = vec![2, 2];
        let result = intersection(nums1, nums2);
        println!("Intersection: {:?}", result);
        assert_eq!(result, vec![2]);
    }

    #[test]
    fn test_intersection_2() {
        let nums1 = vec![4, 9, 5];
        let nums2 = vec![9, 4, 9, 8, 4];
        let result = intersection(nums1, nums2);
        println!("Intersection: {:?}", result);
        assert_eq!(result, vec![4, 9]);
    }

    #[test]
    fn test_intersection_3() {
        let nums1 = vec![1, 2, 3];
        let nums2 = vec![4, 5, 6];
        let result = intersection(nums1, nums2);
        println!("Intersection: {:?}", result);
        assert_eq!(result, Vec::<i32>::new());
    }

    #[test]
    fn test_intersection_4() {
        let nums1 = vec![1, 1, 1];
        let nums2 = vec![1, 1, 1];
        let result = intersection(nums1, nums2);
        println!("Intersection: {:?}", result);
        assert_eq!(result, vec![1]);
    }

    #[test]
    fn test_intersection_5() {
        let nums1 = vec![10, 20, 30];
        let nums2 = vec![30, 40, 50];
        let result = intersection(nums1, nums2);
        println!("Intersection: {:?}", result);
        assert_eq!(result, vec![30]);
    }
}
