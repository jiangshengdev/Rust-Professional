pub fn dp_rec_mc(amount: u32) -> u32 {
    // 针对输入金额为0时直接返回结果
    if amount == 0 {
        return 0;
    }

    // 创建用于存放中间结果的向量，索引代表金额，初始值均为 None
    let mut memo = vec![None; (amount + 1) as usize];

    // 定义递归函数计算最少找零纸币数
    fn rec(a: u32, memo: &mut Vec<Option<u32>>) -> u32 {
        // 金额为0时，无需找零
        if a == 0 {
            return 0;
        }

        // 若 memo 中已有计算结果，则直接返回
        if let Some(result) = memo[a as usize] {
            return result;
        }

        // 币种数组
        let coins = [1, 2, 5, 10, 20, 30, 50, 100];

        // 初始化最小纸币数为最大值，用于后续比较
        let mut min_coins = u32::MAX;

        // 遍历每个币种，计算使用该币种后的找零数量
        for &coin in &coins {
            // 如果当前币种可以用于找零，则尝试计算剩余金额
            if coin <= a {
                // 递归计算剩余金额的找零数量，并加上当前使用的纸币
                let count = rec(a - coin, memo) + 1;
                // 若当前方案更优，则更新最小纸币数
                if count < min_coins {
                    min_coins = count;
                }
            }
        }

        // 将计算结果存入 memo 中
        memo[a as usize] = Some(min_coins);
        min_coins
    }

    // 调用递归函数计算最终的最小找零纸币数
    rec(amount, &mut memo)
}
