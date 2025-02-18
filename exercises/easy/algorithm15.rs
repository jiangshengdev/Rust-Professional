/*
    Longest Substring Without Repeating Characters
    Given a string, find the length of the longest substring without repeating characters.
    The substring must not contain any duplicate characters, and its length should be maximized.

    You need to implement the function `longest_substring_without_repeating_chars(s: String) -> i32`.
    The function should return the length of the longest substring without repeating characters.

    Hint: Consider using the sliding window technique to efficiently solve this problem in O(n) time complexity.
*/
/*
    最长无重复字符子串
    给定一个字符串，找到其中不包含重复字符的最长子串的长度。
    该子串不能包含任何重复字符，并且其长度应最大化。

    你需要实现函数 `longest_substring_without_repeating_chars(s: String) -> i32`。
    此函数应返回最长无重复字符子串的长度。

    提示：考虑使用滑动窗口技术，以 O(n) 时间复杂度高效解决此问题。
*/

use std::fmt::{self, Display, Formatter};

pub fn longest_substring_without_repeating_chars(s: String) -> i32 {
    // 将字符串转换为字符数组，便于使用索引访问字符
    let chars: Vec<char> = s.chars().collect();
    // 创建哈希表记录字符上次出现的位置
    let mut map = std::collections::HashMap::<char, usize>::new();
    // 用于记录当前窗口开始位置
    let mut start = 0;
    // 记录当前无重复字符子串的最大长度
    let mut max_length = 0;
    // 遍历字符数组获取字符及其索引
    for (i, &c) in chars.iter().enumerate() {
        // 如果字符在当前窗口内已出现，则调整窗口开始位置
        if let Some(&prev_index) = map.get(&c) {
            if prev_index >= start {
                // 当前窗口长度与记录的最大长度作比较，更新最大长度
                max_length = max_length.max(i - start);
                // 更新窗口起始位置为之前出现字符的下一个位置
                start = prev_index + 1;
            }
        }
        // 记录当前字符的索引位置，帮助后续判断重复情况
        map.insert(c, i);
    }
    // 最后一次更新窗口长度并比较最大值
    max_length = max_length.max(chars.len() - start);
    max_length as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_longest_substring_1() {
        let s = "abcabcbb".to_string();
        let result = longest_substring_without_repeating_chars(s);
        println!("Length of longest substring: {}", result);
        assert_eq!(result, 3); // "abc"
    }

    #[test]
    fn test_longest_substring_2() {
        let s = "bbbbb".to_string();
        let result = longest_substring_without_repeating_chars(s);
        println!("Length of longest substring: {}", result);
        assert_eq!(result, 1); // "b"
    }

    #[test]
    fn test_longest_substring_3() {
        let s = "pwwkew".to_string();
        let result = longest_substring_without_repeating_chars(s);
        println!("Length of longest substring: {}", result);
        assert_eq!(result, 3); // "wke"
    }

    #[test]
    fn test_longest_substring_4() {
        let s = "".to_string();
        let result = longest_substring_without_repeating_chars(s);
        println!("Length of longest substring: {}", result);
        assert_eq!(result, 0); // Empty string
    }

    #[test]
    fn test_longest_substring_5() {
        let s = "abcde".to_string();
        let result = longest_substring_without_repeating_chars(s);
        println!("Length of longest substring: {}", result);
        assert_eq!(result, 5); // "abcde"
    }
}
