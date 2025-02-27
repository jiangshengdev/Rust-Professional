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
/*
    字母异位词检测
    给定两个字符串，检查它们是否互为字母异位词。
    字母异位词是通过重新排列另一个字符串的字母形成的单词或短语，
    且精确使用所有原始字母一次。
    这些字符串可能包含空格或标点，但在检测时需要忽略它们。

    你需要实现函数 `are_anagrams(s1: String, s2: String) -> bool`。
    如果两个字符串互为字母异位词，则函数应返回 `true`，否则返回 `false`。

    提示：在检测前考虑通过移除非字母字符并转换为小写来标准化字符串。
*/

use std::collections::HashMap;

pub fn are_anagrams(s1: String, s2: String) -> bool {
    let mut counts: HashMap<char, i32> = HashMap::new();

    // 遍历第一个字符串，将所有字母转为小写后计数（兼容多语言）
    for ch in s1.chars() {
        if ch.is_alphabetic() {
            for lower in ch.to_lowercase() {
                *counts.entry(lower).or_insert(0) += 1;
            }
        }
    }

    // 遍历第二个字符串，对应字母计数减少
    for ch in s2.chars() {
        if ch.is_alphabetic() {
            for lower in ch.to_lowercase() {
                *counts.entry(lower).or_insert(0) -= 1;
            }
        }
    }

    // 检查所有字母计数是否均为 0
    counts.values().all(|&count| count == 0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_anagram_1() {
        let s1 = "listen".to_string();
        let s2 = "silent".to_string();
        let result = are_anagrams(s1, s2);
        println!("Are anagrams: {}", result);
        assert!(result);
    }

    #[test]
    fn test_anagram_2() {
        let s1 = "evil".to_string();
        let s2 = "vile".to_string();
        let result = are_anagrams(s1, s2);
        println!("Are anagrams: {}", result);
        assert!(result);
    }

    #[test]
    fn test_anagram_3() {
        let s1 = "hello".to_string();
        let s2 = "world".to_string();
        let result = are_anagrams(s1, s2);
        println!("Are anagrams: {}", result);
        assert!(!result);
    }

    #[test]
    fn test_anagram_4() {
        let s1 = "Clint Eastwood".to_string();
        let s2 = "Old West Action".to_string();
        let result = are_anagrams(s1, s2);
        println!("Are anagrams: {}", result);
        assert!(result);
    }

    #[test]
    fn test_anagram_5() {
        let s1 = "Astronomer".to_string();
        let s2 = "Moon starer".to_string();
        let result = are_anagrams(s1, s2);
        println!("Are anagrams: {}", result);
        assert!(result);
    }
}
