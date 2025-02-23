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
    factorize(number, &mut factors);
    // 从分解出的因子中选取最大的值返回
    *factors.iter().max().unwrap()
}

/// 对数字进行质因数分解
///
/// # 参数
/// n - 需要分解的数字
/// factors - 用于存放质因数的向量
///
/// # 返回值
/// 无返回值，结果通过 factors 参数传递
fn factorize(n: u128, factors: &mut Vec<u128>) {
    // 当 n 分解到 1 时结束递归
    if n == 1 {
        return;
    }
    // 如果 n 是质数，则直接加入因子集合后结束递归
    if is_prime(n) {
        factors.push(n);
        return;
    }
    // 使用 Pollard Rho 算法查找 n 的非平凡因子
    let factor = pollard_rho(n);
    // 对找到的因子递归分解
    factorize(factor, factors);
    // 对 n 除去该因子后的结果进行递归分解
    factorize(n / factor, factors)
}

/// 通过 Pollard Rho 算法寻找非平凡因子
///
/// # 参数
/// n - 需要因数分解的数字
///
/// # 返回值
/// 返回 n 的一个非平凡因子
fn pollard_rho(n: u128) -> u128 {
    // 如果 n 为偶数，则返回 2
    if n % 2 == 0 {
        return 2;
    }
    // 初始化 x 和 y 用于算法迭代
    let mut x = 2;
    let mut y = 2;
    // 初始参数 c 用于函数 f(x)=x^2+c
    let mut c = 1;
    // 变量 d 用于存储 gcd 结果，初始为1
    let mut d = 1;
    // 循环直到找到非平凡因子（d 不再等于 1）
    while d == 1 {
        // 更新 x 值
        x = (mod_mul(x, x, n) + c) % n;
        // 第一次更新 y 值
        y = (mod_mul(y, y, n) + c) % n;
        // 第二次更新 y 值，加速得到周期
        y = (mod_mul(y, y, n) + c) % n;
        // 计算 x 与 y 的差值绝对值
        let diff = if x > y { x - y } else { y - x };
        // 计算差值与 n 的最大公约数 d
        d = gcd(diff, n);
        // 若 d 等于 n，说明本次迭代未找到有效因子，需要调整参数重新开始
        if d == n {
            c += 1;
            x = 2;
            y = 2;
            d = 1;
        }
    }
    // 返回找到的非平凡因子 d
    d
}

/// 计算两个数字的最大公约数
///
/// # 参数
/// a - 第一个数字，类型为 u128
/// b - 第二个数字，类型为 u128
///
/// # 返回值
/// 返回 a 和 b 的最大公约数
fn gcd(mut a: u128, mut b: u128) -> u128 {
    // 当 b 不为 0 时不断计算余数
    while b != 0 {
        // 保存 a 对 b 取余的结果
        let temp = a % b;
        // 将 a 更新为 b
        a = b;
        // 将 b 更新为余数
        b = temp;
    }
    // b 为 0 时，a 即为最大公约数
    a
}

/// 计算 (a * b) % m，防止中间乘法溢出
///
/// # 参数
/// a - 被乘数，类型为 u128
/// b - 乘数，类型为 u128
/// m - 模数，类型为 u128
///
/// # 返回值
/// 返回 (a * b) % m 的结果
fn mod_mul(mut a: u128, mut b: u128, m: u128) -> u128 {
    // 初始化结果，用于累加乘法结果
    let mut res = 0;
    // 将 a 限制在模 m 范围内
    a %= m;
    // 当 b 大于 0 时逐位检查
    while b > 0 {
        // 若 b 的最低位为 1，则将 a 加入结果
        if b & 1 == 1 {
            res = (res + a) % m;
        }
        // 将 a 叠加自身，并取模防止溢出
        a = (a + a) % m;
        // 将 b 右移一位
        b >>= 1;
    }
    // 返回最终的乘法模结果
    res
}

/// 计算 (base ^ exp) % m，采用二进制快速幂算法
///
/// # 参数
/// base - 底数，类型为 u128
/// exp - 指数，类型为 u128
/// m - 模数，类型为 u128
///
/// # 返回值
/// 返回 (base ^ exp) % m 的结果
fn mod_pow(mut base: u128, mut exp: u128, m: u128) -> u128 {
    // 初始化结果为 1
    let mut result = 1;
    // 将底数先取模 m，防止后续计算溢出
    base %= m;
    // 当指数不为零时不断计算
    while exp > 0 {
        // 若当前二进制位为 1，累乘底数到结果上
        if exp & 1 == 1 {
            result = mod_mul(result, base, m);
        }
        // 平方底数并取模
        base = mod_mul(base, base, m);
        // 右移指数一位
        exp >>= 1;
    }
    // 返回快速幂计算结果
    result
}

/// 判断一个数字是否为质数
///
/// # 参数
/// n - 需要判断的数字，类型为 u128
///
/// # 返回值
/// 如果 n 为质数则返回 true，否则返回 false
fn is_prime(n: u128) -> bool {
    // 若 n 小于 2，则非质数
    if n < 2 {
        return false;
    }
    // 若 n 是 2 或 3，则直接确定为质数
    if n == 2 || n == 3 {
        return true;
    }
    // 排除偶数和可被 3 整除的数字
    if n % 2 == 0 || n % 3 == 0 {
        return false;
    }
    // 将 n - 1 转换为 d * 2^s 形式，为 Miller-Rabin 做准备
    let mut d = n - 1;
    let mut s = 0;
    // 提取 d 中的 2 因子，同时记录2的次数 s
    while d % 2 == 0 {
        d /= 2;
        s += 1;
    }
    // 固定一组测试基用于 Miller-Rabin 判断
    let test_bases = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37];
    // 对每个测试基进行迭代
    for &a in test_bases.iter() {
        // 如果当前基大于或等于 n，则跳过测试
        if a >= n {
            continue;
        }
        // 计算 a^d mod n 作为检测值
        let mut x = mod_pow(a, d, n);
        // 若计算结果为 1 或 n-1，则 n 通过本次检测
        if x == 1 || x == n - 1 {
            continue;
        }
        // 默认设置标记为复合数
        let mut composite = true;
        // 进行 s - 1 次平方迭代检测
        for _ in 0..s - 1 {
            // 将 x 平方更新并取模 n
            x = mod_mul(x, x, n);
            // 如果 x 等于 n-1，则 n 有可能是质数
            if x == n - 1 {
                composite = false;
                break;
            }
        }
        // 如果经过所有检测仍为复合数，则返回 false
        if composite {
            return false;
        }
    }
    // 所有测试均通过，返回 true 表示 n 为质数
    true
}
