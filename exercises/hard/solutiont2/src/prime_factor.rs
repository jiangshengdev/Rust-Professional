pub fn find_max_prime_factor(number: u128) -> u128 {
    // 对于输入小于等于1，直接返回输入值
    if number <= 1 {
        return number;
    }
    let mut factors = Vec::new();
    // 递归分解质因数
    factorize(number, &mut factors);
    // 选出所有因子中最大的那个
    *factors.iter().max().unwrap()
}

fn factorize(n: u128, factors: &mut Vec<u128>) {
    // 当数值为1时结束分解
    if n == 1 {
        return;
    }
    // 判断 n 是否是素数
    if is_prime(n) {
        factors.push(n);
        return;
    }
    // 使用 Pollard Rho 算法寻找非平凡因子
    let factor = pollard_rho(n);
    // 递归分解找到的因子
    factorize(factor, factors);
    // 对剩余部分进行因子分解
    factorize(n / factor, factors);
}

fn pollard_rho(n: u128) -> u128 {
    // 如果 n 为偶数，最小因子为2
    if n % 2 == 0 {
        return 2;
    }
    let mut x = 2;
    let mut y = 2;
    let mut c = 1;
    let mut d = 1;
    // 利用 Pollard Rho 算法不断寻找公因数
    while d == 1 {
        x = (mod_mul(x, x, n) + c) % n;
        y = (mod_mul(y, y, n) + c) % n;
        y = (mod_mul(y, y, n) + c) % n;
        // 计算 x 与 y 差值的最大公约数
        let diff = if x > y { x - y } else { y - x };
        d = gcd(diff, n);
        // 若 d 等于 n，说明本轮未寻找成功，调整参数重新开始
        if d == n {
            c += 1;
            x = 2;
            y = 2;
            d = 1;
        }
    }
    d
}

fn gcd(mut a: u128, mut b: u128) -> u128 {
    while b != 0 {
        let temp = a % b;
        a = b;
        b = temp;
    }
    a
}

fn mod_mul(mut a: u128, mut b: u128, m: u128) -> u128 {
    let mut res = 0;
    a %= m;
    // 通过逐位处理 b 的每一位，防止溢出
    while b > 0 {
        if b & 1 == 1 {
            res = (res + a) % m;
        }
        a = (a + a) % m;
        b >>= 1;
    }
    res
}

fn mod_pow(mut base: u128, mut exp: u128, m: u128) -> u128 {
    let mut result = 1;
    base %= m;
    // 通过二进制展开快速计算幂
    while exp > 0 {
        if exp & 1 == 1 {
            result = mod_mul(result, base, m);
        }
        base = mod_mul(base, base, m);
        exp >>= 1;
    }
    result
}

fn is_prime(n: u128) -> bool {
    // 小于2的数不是素数
    if n < 2 {
        return false;
    }
    // 2和3是素数
    if n == 2 || n == 3 {
        return true;
    }
    // 排除偶数和能整除3的情况
    if n % 2 == 0 || n % 3 == 0 {
        return false;
    }
    // 将 n - 1 表示为 d * 2^s
    let mut d = n - 1;
    let mut s = 0;
    while d % 2 == 0 {
        d /= 2;
        s += 1;
    }
    // 使用固定的几个基进行 Miller-Rabin 判定
    let test_bases = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37];
    for &a in test_bases.iter() {
        if a >= n {
            continue;
        }
        let mut x = mod_pow(a, d, n);
        if x == 1 || x == n - 1 {
            continue;
        }
        let mut composite = true;
        // 对每次平方检测进行循环
        for _ in 0..s - 1 {
            x = mod_mul(x, x, n);
            if x == n - 1 {
                composite = false;
                break;
            }
        }
        if composite {
            return false;
        }
    }
    true
}
