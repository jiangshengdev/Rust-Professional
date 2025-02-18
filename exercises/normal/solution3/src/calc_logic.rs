pub fn new_birthday_probability(n: u32) -> f64 {
    // 检查人数是否超过 365，如果超过，则必定有重复生日
    if n > 365 {
        return 1.0;
    }

    // 初始化所有人生日各不相同的概率为 1.0
    let mut distinct = 1.0;

    // 遍历每个人，累乘计算每人生日与前面的人不同的概率
    for i in 0..n {
        // 将当前步的概率加入累计结果中，每一步都按浮点数计算
        distinct *= (365 - i) as f64 / 365.0;
    }

    // 计算至少存在两人同一天过生日的概率
    let prob = 1.0 - distinct;

    // 返回最终计算结果，四舍五入保留四位小数
    (prob * 10000.0).round() / 10000.0
}
