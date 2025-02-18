pub fn goldbach_conjecture() -> String {
    // 定义一个函数判断是否为素数
    fn is_prime(n: u64) -> bool {
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

    // 用来存放找出的不符合表示方式的候选数
    let mut results = Vec::new();
    // 从 9 开始，因为最小奇合数是 9
    let mut candidate = 9;

    loop {
        // 如果为奇数且不是素数，则为奇合数
        if candidate % 2 == 1 && !is_prime(candidate) {
            // 标记是否能用素数加上平方的两倍表示
            let mut valid = false;
            // 遍历所有 2 到 candidate 之间可能的素数
            let mut p = 2;
            while p < candidate {
                if is_prime(p) {
                    // 计算差值
                    let remainder = candidate - p;
                    // 保证差值可以被2整除
                    if remainder % 2 == 0 {
                        let half = remainder / 2;
                        // 计算 half 的整数平方根
                        let mut s = 0;
                        // 从0开始尝试判断是否有平方数的匹配
                        loop {
                            // 当平方大于 half 时退出循环
                            if s * s > half {
                                break;
                            }
                            // 如果找到满足条件的平方，则标记为能表示，退出循环
                            if s * s == half {
                                valid = true;
                                break;
                            }
                            s += 1;
                        }
                    }
                    // 如果已经找到满足条件的表示方法，则退出对素数的检查
                    if valid {
                        break;
                    }
                }
                p += 1;
            }
            // 如果 candidate 无法表示为素数和一个平方的两倍之和，则记录该候选数
            if !valid {
                results.push(candidate);
                // 当找齐两个则退出循环
                if results.len() == 2 {
                    break;
                }
            }
        }
        // 候选数递增，每次加2的步长确保只检查奇数
        candidate += 2;
    }

    // 将找到的两个候选数使用逗号拼接为字符串返回
    format!("{},{}", results[0], results[1])
}
