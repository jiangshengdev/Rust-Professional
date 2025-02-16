/*
    Merge Intervals
    Given an array of intervals where each interval is represented by a pair of integers [start, end],
    merge all overlapping intervals and return a list of non-overlapping intervals.

    The intervals are inclusive, meaning the interval [start, end] includes both start and end points.

    You need to implement the function `merge_intervals(intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>>`.
    The function should return a vector containing all the merged intervals.

    Hint: You can start by sorting the intervals by their starting point and then merge them one by one.
*/
/*
    合并区间
    给定一个区间数组，其中每个区间由一对整数 [start, end] 表示，
    合并所有重叠的区间，并返回一个不重叠区间的列表。

    区间是闭区间，即区间 [start, end] 包括起始和结束点。

    你需要实现函数 `merge_intervals(intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>>`。
    函数应返回一个包含所有合并区间的向量。

    提示：你可以先按照区间的起始点排序，然后逐个合并重叠的区间。
*/

use std::fmt::{self, Display, Formatter};

pub fn merge_intervals(intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    // 若区间列表为空，直接返回空数组
    if intervals.is_empty() {
        return Vec::new();
    }

    // 复制区间列表，并按照区间的起始点排序
    let mut sorted_intervals = intervals;
    sorted_intervals.sort_by(|a, b| a[0].cmp(&b[0]));

    // 初始化合并后的区间数组
    let mut merged: Vec<Vec<i32>> = Vec::new();

    // 遍历每个排序后的区间进行合并
    for interval in sorted_intervals {
        // 如果合并数组为空，则直接将当前区间加入结果中
        if merged.is_empty() {
            merged.push(interval);
        } else {
            // 取出结果中最后一个区间用于比较
            let last = merged.last_mut().unwrap();
            // 如果当前区间与最后区间有重叠，则更新最后区间的结束点
            if interval[0] <= last[1] {
                last[1] = std::cmp::max(last[1], interval[1]);
            } else {
                // 如果无重叠，则将当前区间加入结果中
                merged.push(interval);
            }
        }
    }

    merged
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_merge_intervals_1() {
        let intervals = vec![vec![1, 3], vec![2, 6], vec![8, 10], vec![15, 18]];
        let result = merge_intervals(intervals);
        println!("Merged intervals: {:?}", result);
        assert_eq!(result, vec![vec![1, 6], vec![8, 10], vec![15, 18]]);
    }

    #[test]
    fn test_merge_intervals_2() {
        let intervals = vec![vec![1, 4], vec![4, 5]];
        let result = merge_intervals(intervals);
        println!("Merged intervals: {:?}", result);
        assert_eq!(result, vec![vec![1, 5]]);
    }

    #[test]
    fn test_merge_intervals_3() {
        let intervals = vec![vec![1, 4], vec![0, 4]];
        let result = merge_intervals(intervals);
        println!("Merged intervals: {:?}", result);
        assert_eq!(result, vec![vec![0, 4]]);
    }

    #[test]
    fn test_merge_intervals_4() {
        let intervals = vec![vec![1, 10], vec![2, 6], vec![8, 10]];
        let result = merge_intervals(intervals);
        println!("Merged intervals: {:?}", result);
        assert_eq!(result, vec![vec![1, 10]]);
    }

    #[test]
    fn test_merge_intervals_5() {
        let intervals = vec![vec![1, 2], vec![3, 5], vec![4, 7], vec![8, 10]];
        let result = merge_intervals(intervals);
        println!("Merged intervals: {:?}", result);
        assert_eq!(result, vec![vec![1, 2], vec![3, 7], vec![8, 10]]);
    }
}
