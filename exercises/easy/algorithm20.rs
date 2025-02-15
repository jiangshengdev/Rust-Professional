/*
    Sum of Two Integers
    Given two integers, calculate their sum without using the `+` operator.
    You need to implement the function `get_sum(a: i32, b: i32) -> i32`.
    The function should return the sum of the two integers `a` and `b`.

    Hint: You can solve this problem using bitwise operations.
*/
/*
    两个整数之和
    给定两个整数，但不能使用 `+` 运算符，计算它们的和。
    你需要实现函数 `get_sum(a: i32, b: i32) -> i32`。
    该函数应返回两个整数 `a` 和 `b` 的和。

    提示：你可以使用位运算来解决此问题。
*/

use std::fmt::{self, Display, Formatter};

pub fn get_sum(a: i32, b: i32) -> i32 {
    // TODO: Implement the logic to calculate the sum of two integers without using `+`
    // TODO: 使用位运算实现不使用 `+` 运算符计算两个整数之和的逻辑
    0 // Placeholder return value
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum_1() {
        let result = get_sum(1, 2);
        println!("Sum of 1 and 2: {}", result);
        assert_eq!(result, 3);
    }

    #[test]
    fn test_sum_2() {
        let result = get_sum(-1, 1);
        println!("Sum of -1 and 1: {}", result);
        assert_eq!(result, 0);
    }

    #[test]
    fn test_sum_3() {
        let result = get_sum(100, 200);
        println!("Sum of 100 and 200: {}", result);
        assert_eq!(result, 300);
    }

    #[test]
    fn test_sum_4() {
        let result = get_sum(-50, -50);
        println!("Sum of -50 and -50: {}", result);
        assert_eq!(result, -100);
    }

    #[test]
    fn test_sum_5() {
        let result = get_sum(0, 0);
        println!("Sum of 0 and 0: {}", result);
        assert_eq!(result, 0);
    }
}
