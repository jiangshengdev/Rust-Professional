/*
    Nth Fibonacci Number
    Implement a function to calculate the `n`th Fibonacci number.
    The Fibonacci sequence is defined as follows:
    F(0) = 0, F(1) = 1, F(n) = F(n-1) + F(n-2) for n > 1.

    You need to implement the function `fib(n: i32) -> i32` to return the `n`th Fibonacci number.

    Hint: Consider using matrix exponentiation to solve the problem in O(log n) time complexity.
*/
/*
    N阶斐波那契数
    实现一个函数来计算第 n 个斐波那契数。
    斐波那契数列定义如下：
    F(0) = 0, F(1) = 1，当 n > 1 时，F(n) = F(n-1) + F(n-2)。

    你需要实现函数 `fib(n: i32) -> i32` 来返回第 n 个斐波那契数。

    提示：考虑使用矩阵快速幂算法以 O(log n) 时间复杂度求解。
*/

use std::fmt::{self, Display, Formatter};

pub fn fib(n: i32) -> i32 {
    // 判断 n 是否为 0
    if n == 0 {
        // 返回 0 作为斐波那契数列第 0 项
        return 0;
    }

    // 定义二维矩阵相乘函数，矩阵使用 2x2 数组表示
    fn mat_mul(a: [[i32; 2]; 2], b: [[i32; 2]; 2]) -> [[i32; 2]; 2] {
        // 初始化结果矩阵为全 0
        let mut c = [[0, 0], [0, 0]];
        // 计算矩阵第一行第一列
        c[0][0] = a[0][0] * b[0][0] + a[0][1] * b[1][0];
        // 计算矩阵第一行第二列
        c[0][1] = a[0][0] * b[0][1] + a[0][1] * b[1][1];
        // 计算矩阵第二行第一列
        c[1][0] = a[1][0] * b[0][0] + a[1][1] * b[1][0];
        // 计算矩阵第二行第二列
        c[1][1] = a[1][0] * b[0][1] + a[1][1] * b[1][1];
        // 返回乘法结果矩阵
        c
    }

    // 定义矩阵快速幂函数，使用迭代计算
    fn mat_pow(mut mat: [[i32; 2]; 2], mut power: i32) -> [[i32; 2]; 2] {
        // 初始化结果为单位矩阵
        let mut result = [[1, 0], [0, 1]];
        // 当幂大于 0 时循环计算
        while power > 0 {
            // 如果幂为奇数，将结果矩阵和当前矩阵相乘
            if power % 2 == 1 {
                result = mat_mul(result, mat);
            }
            // 当前矩阵自乘，计算平方
            mat = mat_mul(mat, mat);
            // 将幂除以 2
            power /= 2;
        }
        // 返回最终矩阵结果
        result
    }

    // 定义斐波那契数列的基矩阵
    let base = [[1, 1], [1, 0]];
    // 计算基矩阵的 (n-1) 次幂
    let res = mat_pow(base, n - 1);
    // 返回结果矩阵的 [0][0] 元素，即为斐波那契数列第 n 项
    res[0][0]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fib_1() {
        let result = fib(0);
        println!("Fibonacci of 0: {}", result);
        assert_eq!(result, 0);
    }

    #[test]
    fn test_fib_2() {
        let result = fib(1);
        println!("Fibonacci of 1: {}", result);
        assert_eq!(result, 1);
    }

    #[test]
    fn test_fib_3() {
        let result = fib(2);
        println!("Fibonacci of 2: {}", result);
        assert_eq!(result, 1);
    }

    #[test]
    fn test_fib_4() {
        let result = fib(3);
        println!("Fibonacci of 3: {}", result);
        assert_eq!(result, 2);
    }

    #[test]
    fn test_fib_5() {
        let result = fib(10);
        println!("Fibonacci of 10: {}", result);
        assert_eq!(result, 55);
    }

    #[test]
    fn test_fib_6() {
        let result = fib(20);
        println!("Fibonacci of 20: {}", result);
        assert_eq!(result, 6765);
    }
}
