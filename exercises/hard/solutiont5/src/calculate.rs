/// 计算初步退休日期，基于出生日期和基础退休年龄
///
/// # 参数
/// * `birth_year` - 出生年份
/// * `birth_month` - 出生月份
/// * `base_age` - 基础退休年龄
///
/// # 返回
/// 返回一个元组 `(retire_year, retire_month)`
/// * `retire_year` - 初步计算的退休年份
/// * `retire_month` - 初步计算的退休月份
pub fn calculate_initial_retirement_date(
    birth_year: i32,
    birth_month: i32,
    base_age: i32,
) -> (i32, i32) {
    // 初步计算退休年份和退休月份
    let retire_year = birth_year + base_age;
    let retire_month = birth_month;
    (retire_year, retire_month)
}
