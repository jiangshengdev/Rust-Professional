/*
    Palindrome Check
    Given a string, check if it is a palindrome (i.e., it reads the same forward and backward).
    The solution should ignore case differences and non-alphabetical characters.

    You need to implement the function `is_palindrome(s: String) -> bool`.
    The function should return `true` if the string is a palindrome, and `false` otherwise.

    Hint: Consider normalizing the string by converting it to lowercase and removing non-alphabetical characters before checking.
*/
/*
    中文翻译:
    回文检查
    给定一个字符串，检查它是否为回文（即正读与倒读相同）。
    实现时应忽略大小写差异和非字母字符。

    你需要实现函数 `is_palindrome(s: String) -> bool`。
    如果字符串是回文则返回 `true`，否则返回 `false`。

    提示：在检查之前，可以通过将字符串转换为小写并去除非字母字符来归一化字符串。
*/

use std::fmt::{self, Display, Formatter};

pub fn is_palindrome(s: String) -> bool {
    // 筛选非字母字符，并将字母转换成小写后存入 vector 中
    let filtered: Vec<char> = s
        .chars()
        .filter(|c| c.is_alphabetic())
        .map(|c| c.to_ascii_lowercase())
        .collect();

    // 初始化左右两个指针用于检查对应字符
    let mut left = 0;
    let mut right = filtered.len().saturating_sub(1);

    // 左右指针未交叉时
    while left < right {
        // 判断左右指针对应的字符是否一致
        if filtered[left] != filtered[right] {
            // 不一致则立即返回 false
            return false;
        }
        // 当前字符一致，左指针右移
        left += 1;

        // 当前字符一致，右指针左移
        right -= 1;
    }

    // 所有字符都相对应，返回 true
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_palindrome_1() {
        let s = "A man, a plan, a canal, Panama".to_string();
        let result = is_palindrome(s);
        println!("Is palindrome: {}", result);
        assert_eq!(result, true);
    }

    #[test]
    fn test_palindrome_2() {
        let s = "Racecar".to_string();
        let result = is_palindrome(s);
        println!("Is palindrome: {}", result);
        assert_eq!(result, true);
    }

    #[test]
    fn test_palindrome_3() {
        let s = "Hello, World!".to_string();
        let result = is_palindrome(s);
        println!("Is palindrome: {}", result);
        assert_eq!(result, false);
    }

    #[test]
    fn test_palindrome_4() {
        let s = "No 'x' in Nixon".to_string();
        let result = is_palindrome(s);
        println!("Is palindrome: {}", result);
        assert_eq!(result, true);
    }

    #[test]
    fn test_palindrome_5() {
        let s = "Was it a car or a cat I saw?".to_string();
        let result = is_palindrome(s);
        println!("Is palindrome: {}", result);
        assert_eq!(result, true);
    }
}
