use crate::{prime, sum};

pub fn goldbach_conjecture() -> String {
    // 存放不符合表示方式的候选数
    let mut results = Vec::new();
    // 9 为最小奇合数
    let mut candidate = 9;

    loop {
        // 判断奇数且非素数
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
