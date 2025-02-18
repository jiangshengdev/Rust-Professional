pub fn odd_fibnacci_sum(threshold: u32) -> u32 {
    // 初始化奇数和累加器
    let mut sum = 0;

    // 设置 Fibonacci 数列初始值
    let mut a = 0;
    let mut b = 1;

    // 遍历 Fibonacci 数列直到当前值超出阈值
    while a < threshold {
        // 判断当前值是否为奇数
        if a % 2 != 0 {
            // 如果为奇数，则累加到和中
            sum += a;
        }

        // 计算下一个 Fibonacci 数
        let c = a + b;

        // 更新当前数值为下一个数
        a = b;

        // 更新下一个数值为计算结果
        b = c;
    }

    // 返回累加的奇数之和
    sum
}
