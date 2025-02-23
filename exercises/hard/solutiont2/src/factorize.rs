use crate::{is_prime, pollard_rho};

/// 对数字进行质因数分解
///
/// # 参数
/// n - 需要分解的数字
/// factors - 用于存放质因数的向量
///
/// # 返回值
/// 无返回值，结果通过 factors 参数传递
pub fn factorize(n: u128, factors: &mut Vec<u128>) {
    // 当 n 分解到 1 时结束递归
    if n == 1 {
        return;
    }
    // 如果 n 是质数，则直接加入因子集合后结束递归
    if is_prime::is_prime(n) {
        factors.push(n);
        return;
    }
    // 使用 Pollard Rho 算法查找 n 的非平凡因子
    let factor = pollard_rho::pollard_rho(n);
    // 对找到的因子递归分解
    factorize(factor, factors);
    // 对 n 除去该因子后的结果进行递归分解
    factorize(n / factor, factors)
}
