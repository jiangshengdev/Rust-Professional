/// 更新退休日期，根据延迟规则调整退休年份和月份
///
/// # 参数
/// * `retire_year` - 初始退休年份 (整数)
/// * `retire_month` - 初始退休月份 (整数)
/// * `delay_interval` - 延迟间隔计算参数 (整数)
/// * `target_age` - 目标退休年龄 (整数)
/// * `base_age` - 基础退休年龄 (整数)
///
/// # 返回
/// 返回一个元组 `(new_year, new_month, delay_months)`
/// * `new_year` - 更新后的退休年份
/// * `new_month` - 更新后的退休月份
/// * `delay_months` - 实际延迟的月份数
pub fn update_retirement_date(
    retire_year: i32,
    retire_month: i32,
    delay_interval: i32,
    target_age: i32,
    base_age: i32,
) -> (i32, i32, i32) {
    // 计算自2025年起累计的月份数
    let months_from_2025 = (retire_year - 2025) * 12 + retire_month;

    // 通过平滑取整计算延迟月数
    let computed_delay = (months_from_2025 - 1 + delay_interval) / delay_interval;

    // 计算允许的最大延迟（月数）为目标退休年龄和基础退休年龄之差转换而来
    let max_delay = (target_age - base_age) * 12;

    // 取实际延迟月数为较小的值
    let delay_months = if computed_delay > max_delay {
        max_delay
    } else {
        computed_delay
    };

    // 调整退休月份和年份
    let mut new_month = retire_month + delay_months % 12;
    let mut new_year = retire_year + delay_months / 12;
    if new_month > 12 {
        new_month -= 12;
        new_year += 1;
    }

    (new_year, new_month, delay_months)
}
