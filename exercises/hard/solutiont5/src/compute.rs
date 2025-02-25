/// 计算实际退休年龄（单位：年）
///
/// # 参数
/// * `birth_year` - 出生年份
/// * `birth_month` - 出生月份
/// * `retire_year` - 退休年份
/// * `retire_month` - 退休月份
///
/// # 返回
/// 实际退休年龄（浮点数）
pub fn compute_actual_age(
    birth_year: i32,
    birth_month: i32,
    retire_year: i32,
    retire_month: i32,
) -> f64 {
    // 计算从出生到退休共经过的月份数
    let total_months = (retire_year - birth_year) * 12 + (retire_month - birth_month);

    // 将月份转换为年数
    total_months as f64 / 12.0
}
