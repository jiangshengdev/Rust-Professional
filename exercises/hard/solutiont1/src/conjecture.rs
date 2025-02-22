use crate::{prime, sum};

/// 返回格式化后的字符串，包含两个满足指定条件的候选数
///
/// 候选数必须是奇数，并且
/// 检查该数是否既不是素数也不能表示为两个平方数与素数之和
/// 当找到两个符合条件的候选数后，用逗号分割返回
pub fn goldbach_conjecture() -> String {
    // 存放不符合表示方式的候选数
    let mut results = Vec::new();
    // 9 为最小奇合数
    let mut candidate = 9;

    loop {
        // 判断奇数且非素数，以及该数字不能表示为素数与两倍平方数之和
        if candidate % 2 == 1
            && !prime::is_prime(candidate)
            && !sum::is_sum_of_prime_twice_square(candidate)
        {
            results.push(candidate);
            // 找到两个候选数后退出
            if results.len() == 2 {
                break;
            }
        }
        // 每次增加2只检查奇数
        candidate += 2;
    }

    // 用逗号拼接结果返回
    format!("{},{}", results[0], results[1])
}
