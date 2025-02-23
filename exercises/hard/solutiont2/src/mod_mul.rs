/// 计算 (a * b) % m，防止中间乘法溢出
///
/// # 参数
/// a - 被乘数，类型为 u128
/// b - 乘数，类型为 u128
/// m - 模数，类型为 u128
///
/// # 返回值
/// 返回 (a * b) % m 的结果
pub(crate) fn mod_mul(a: u128, b: u128, m: u128) -> u128 {
    // 对 a 进行模 m 归一化
    let a = a % m;

    // 对 b 进行模 m 归一化
    let b = b % m;

    // 尝试直接相乘，避免额外运算
    if let Some(product) = a.checked_mul(b) {
        // 直接返回乘积模 m 的结果
        product % m
    } else {
        // 初始化累加结果为 0
        let mut res = 0;

        // 创建局部变量 a 以便后续计算
        let mut a = a;

        // 创建局部变量 b 以便右移操作
        let mut b = b;

        // 循环，直至 b 为 0
        while b > 0 {
            // 判断 b 的最低位是否为 1
            if b & 1 == 1 {
                // 累加 a 到结果中并取模防止溢出
                res = (res + a) % m;
            } else {
                // 当 b 的最低位不为 1 时，无需累加
            }

            // 将 a 左移一位，并取模 m 防止溢出
            a = (a << 1) % m;

            // 右移 b 以处理下一位
            b >>= 1;
        }

        // 返回最终累加结果
        res
    }
}
