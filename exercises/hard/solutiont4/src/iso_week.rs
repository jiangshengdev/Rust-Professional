use crate::utils;

/// 使用坂本算法计算星期几
///
/// # 参数
/// - `year`: 年份，符合公历年份
/// - `month`: 月份（1 至 12）
/// - `day`: 日
///
/// # 返回值
/// 返回一个整数，表示星期几（1 表示星期一，7 表示星期日）
///
/// # 示例
/// ```
/// let weekday = day_of_week_sakamoto(2025, 2, 20);
/// println!("Weekday: {}", weekday);
/// ```
pub fn day_of_week_sakamoto(year: i32, month: usize, day: i32) -> i32 {
    let mut y = year;
    let mut m = month as i32;
    if m == 1 || m == 2 {
        y -= 1;
        m += 12;
    }
    let k = y % 100;
    let j = y / 100;
    let h = (day + (13 * (m + 1)) / 5 + k + (k / 4) + (j / 4) + 5 * j) % 7;
    (h + 5) % 7 + 1
}

/// 获取上一年度最后一周的 ISO 周数（内部函数）
///
/// # 参数
/// - `year`: 年份
///
/// # 返回值
/// 返回上一年度最后一周的 ISO 周数
fn last_iso_week_number_of_year(year: i32) -> i32 {
    let prev_year_total = if utils::is_leap_year(year) { 366 } else { 365 };
    let prev_dec31_week_day = day_of_week_sakamoto(year, 12, 31);
    let prev_thursday = prev_year_total + (4 - prev_dec31_week_day);
    (prev_thursday - 1) / 7 + 1
}

/// 根据给定日期计算 ISO 周数和周几
///
/// # 参数
/// - `year`: 年份
/// - `month`: 月份（1 至 12）
/// - `day`: 日
///
/// # 返回值
/// 返回一个元组 `(iso_week_number, iso_week_day)`，
/// 其中 `iso_week_number` 为 ISO 周数，`iso_week_day` 为星期几（1 表示星期一，7 表示星期日）
///
/// # 示例
/// ```
/// let (week_number, weekday) = iso_week_number(2025, 2, 20);
/// println!("ISO Week Number: {}, Weekday: {}", week_number, weekday);
/// ```
pub fn iso_week_number(year: i32, month: usize, day: i32) -> (i32, i32) {
    let iso_week_day = day_of_week_sakamoto(year, month, day);
    let (current_day_of_year, total_days) = utils::get_year_info(year, month, day);
    let diff = 4 - iso_week_day;
    let thursday_day_of_year = current_day_of_year + diff;
    if thursday_day_of_year < 1 {
        (last_iso_week_number_of_year(year - 1), iso_week_day)
    } else if thursday_day_of_year > total_days {
        (1, iso_week_day)
    } else {
        ((thursday_day_of_year - 1) / 7 + 1, iso_week_day)
    }
}
