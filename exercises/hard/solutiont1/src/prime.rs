/// 判断一个数是否为素数
///
/// # 参数
///
/// n - 待判断的数字，类型为 u64
///
/// # 返回值
///
/// 如果 n 是素数，返回 true；否则返回 false
pub fn is_prime(n: u64) -> bool {
    // 小于2的数不是素数
    if n < 2 {
        return false;
    }
    // 2 为素数
    if n == 2 {
        return true;
    }
    // 偶数不可能为素数
    if n % 2 == 0 {
        return false;
    }
    // 检查从3开始的奇数因子
    let mut i = 3;
    while i * i <= n {
        if n % i == 0 {
            return false;
        }
        i += 2;
    }
    true
}
