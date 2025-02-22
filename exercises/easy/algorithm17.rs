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

pub fn intersection(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
    // 复制输入数组，避免在排序时修改原始数据
    let mut sorted1 = nums1;
    let mut sorted2 = nums2;

    // 对第一个数组进行排序，确保双指针遍历时有序
    sorted1.sort();

    // 对第二个数组进行排序，确保双指针遍历时有序
    sorted2.sort();

    // 初始化双指针，分别指向两个数组的起始位置
    let mut i = 0;
    let mut j = 0;

    // 创建用于存放交集结果的向量
    let mut result = Vec::new();

    // 当两个指针都未超出各自数组长度时，进入循环进行比较
    while i < sorted1.len() && j < sorted2.len() {
        // 使用 cmp 方法比较当前两个元素的大小
        use std::cmp::Ordering;
        match sorted1[i].cmp(&sorted2[j]) {
            // 如果两个元素相等，表示找到交集元素
            Ordering::Equal => {
                // 检查结果是否为空或上次添加的元素与当前元素不同
                if result.is_empty() || *result.last().unwrap() != sorted1[i] {
                    result.push(sorted1[i]);
                }
                // 同时推进两个指针，继续比较下一个元素
                i += 1;
                j += 1;
            }
            // 如果数组1当前元素较小，则移动数组1的指针
            Ordering::Less => {
                i += 1;
            }
            // 如果数组2当前元素较小，则移动数组2的指针
            Ordering::Greater => {
                j += 1;
            }
        }
    }

    // 返回最终得到的不重复交集结果
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
