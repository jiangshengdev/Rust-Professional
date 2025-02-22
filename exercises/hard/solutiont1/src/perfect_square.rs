// 判断一个数是否为完全平方数
pub fn is_perfect_square(n: u64) -> bool {
    // 计算平方根并向下取整
    let s = (n as f64).sqrt() as u64;
    // 比较平方是否等于原数
    s * s == n
}
