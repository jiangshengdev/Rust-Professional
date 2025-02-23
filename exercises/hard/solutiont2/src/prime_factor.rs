use crate::factorize;

/// 计算输入数字的最大质因数
///
/// # 参数
/// number - 待分解质因数的数字，类型为 u128
///
/// # 返回值
/// 若 number 大于 1，则返回 number 的最大质因数；否则直接返回 number
pub fn find_max_prime_factor(number: u128) -> u128 {
    // 如果输入小于等于 1，则直接返回原数字
    if number <= 1 {
        return number;
    }
    // 创建空向量用于存放分解结果
    let mut factors = Vec::new();
    // 调用递归函数分解质因数
    factorize::factorize(number, &mut factors);
    // 从分解出的因子中选取最大的值返回
    *factors.iter().max().unwrap()
}
