use crate::{mod_mul, mod_pow};

/// 判断一个数字是否为质数
///
/// # 参数
/// n - 需要判断的数字，类型为 u128
///
/// # 返回值
/// 如果 n 为质数则返回 true，否则返回 false
pub fn is_prime(n: u128) -> bool {
    // 若 n 小于 2，则非质数
    if n < 2 {
        return false;
    }
    // 若 n 是 2 或 3，则直接确定为质数
    if n == 2 || n == 3 {
        return true;
    }
    // 排除偶数和可被 3 整除的数字
    if n % 2 == 0 || n % 3 == 0 {
        return false;
    }
    // 将 n - 1 转换为 d * 2^s 形式，为 Miller-Rabin 做准备
    let mut d = n - 1;
    let mut s = 0;
    // 提取 d 中的 2 因子，同时记录2的次数 s
    while d % 2 == 0 {
        d /= 2;
        s += 1;
    }
    // 固定一组测试基用于 Miller-Rabin 判断
    let test_bases = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37];
    // 对每个测试基进行迭代
    for &a in test_bases.iter() {
        // 如果当前基大于或等于 n，则跳过测试
        if a >= n {
            continue;
        }
        // 计算 a^d mod n 作为检测值
        let mut x = mod_pow::mod_pow(a, d, n);
        // 若计算结果为 1 或 n-1，则 n 通过本次检测
        if x == 1 || x == n - 1 {
            continue;
        }
        // 默认设置标记为复合数
        let mut composite = true;
        // 进行 s - 1 次平方迭代检测
        for _ in 0..s - 1 {
            // 将 x 平方更新并取模 n
            x = mod_mul::mod_mul(x, x, n);
            // 如果 x 等于 n-1，则 n 有可能是质数
            if x == n - 1 {
                composite = false;
                break;
            }
        }
        // 如果经过所有检测仍为复合数，则返回 false
        if composite {
            return false;
        }
    }
    // 所有测试均通过，返回 true 表示 n 为质数
    true
}
