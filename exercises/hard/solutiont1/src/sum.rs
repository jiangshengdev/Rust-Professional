use crate::{perfect_square, prime};

// 用来判断能否用素数加平方的两倍表示
pub fn is_sum_of_prime_twice_square(candidate: u64) -> bool {
    // 从最小素数开始判断
    let mut p = 2;
    while p < candidate {
        if prime::is_prime(p) {
            // 判断当前的素数能否满足条件
            if is_candidate_valid(candidate, p) {
                return true;
            }
        }
        p += 1;
    }
    false
}

// 根据候选数与素数计算差值，判断是否满足两倍平方的条件
fn is_candidate_valid(candidate: u64, p: u64) -> bool {
    // 计算候选数与素数的差值
    let remainder = candidate - p;
    if remainder % 2 == 0 {
        // 计算作为平方候选数的值
        let half = remainder / 2;
        if perfect_square::is_perfect_square(half) {
            return true;
        }
    }
    false
}
