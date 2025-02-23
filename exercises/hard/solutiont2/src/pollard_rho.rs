use crate::{gcd, mod_mul};

/// 通过 Pollard Rho 算法寻找非平凡因子
///
/// # 参数
/// n - 需要因数分解的数字
///
/// # 返回值
/// 返回 n 的一个非平凡因子
pub(crate) fn pollard_rho(n: u128) -> u128 {
    // 如果 n 为偶数，则返回 2
    if n % 2 == 0 {
        return 2;
    }
    // 初始化 x 和 y 用于算法迭代
    let mut x = 2;
    let mut y = 2;
    // 初始参数 c 用于函数 f(x)=x^2+c
    let mut c = 1;
    // 变量 d 用于存储 gcd 结果，初始为1
    let mut d = 1;
    // 循环直到找到非平凡因子（d 不再等于 1）
    while d == 1 {
        // 更新 x 值
        x = (mod_mul::mod_mul(x, x, n) + c) % n;
        // 第一次更新 y 值
        y = (mod_mul::mod_mul(y, y, n) + c) % n;
        // 第二次更新 y 值，加速得到周期
        y = (mod_mul::mod_mul(y, y, n) + c) % n;
        // 计算 x 与 y 的差值绝对值
        let diff = if x > y { x - y } else { y - x };
        // 计算差值与 n 的最大公约数 d
        d = gcd::gcd(diff, n);
        // 若 d 等于 n，说明本次迭代未找到有效因子，需要调整参数重新开始
        if d == n {
            c += 1;
            x = 2;
            y = 2;
            d = 1;
        }
    }
    // 返回找到的非平凡因子 d
    d
}
