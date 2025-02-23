use crate::mod_mul;

/// 计算 (base ^ exp) % m，采用二进制快速幂算法
///
/// # 参数
/// base - 底数，类型为 u128
/// exp - 指数，类型为 u128
/// m - 模数，类型为 u128
///
/// # 返回值
/// 返回 (base ^ exp) % m 的结果
pub(crate) fn mod_pow(mut base: u128, mut exp: u128, m: u128) -> u128 {
    // 初始化结果为 1
    let mut result = 1;
    // 将底数先取模 m，防止后续计算溢出
    base %= m;
    // 当指数不为零时不断计算
    while exp > 0 {
        // 若当前二进制位为 1，累乘底数到结果上
        if exp & 1 == 1 {
            result = mod_mul::mod_mul(result, base, m);
        }
        // 平方底数并取模
        base = mod_mul::mod_mul(base, base, m);
        // 右移指数一位
        exp >>= 1;
    }
    // 返回快速幂计算结果
    result
}
