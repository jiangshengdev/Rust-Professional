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
    // 复制输入的年份和月份，用于后续修改
    let mut y = year;
    let mut m = month as i32;

    // 对于1月和2月，将其视为上一年的13月和14月，便于统一计算
    if m == 1 || m == 2 {
        // 调整年份为上一年
        y -= 1;
        // 将月份加12，使1月、2月转变为13月、14月
        m += 12;
    }

    // 计算年份后两位，用于公式中的计算
    let k = y % 100;
    // 计算年份前两位，用于公式中的计算
    let j = y / 100;

    // 使用坂本算法计算中间结果h
    let h = (day + (13 * (m + 1)) / 5 + k + (k / 4) + (j / 4) + 5 * j) % 7;

    // 将计算结果转换为星期表示，其中1代表星期一，7代表星期日
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
    // 判断年份是否为闰年，以确定上一年的总天数
    let prev_year_total = if utils::is_leap_year(year) { 366 } else { 365 };

    // 计算上一年12月31日是星期几
    let prev_dec31_week_day = day_of_week_sakamoto(year, 12, 31);

    // 根据ISO标准，找到上一年最后一周的周四在该年的位置
    let prev_thursday = prev_year_total + (4 - prev_dec31_week_day);

    // 通过计算得到上一年度最后一周的周数
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
    // 计算给定日期对应的星期数
    let iso_week_day = day_of_week_sakamoto(year, month, day);

    // 获取当前日期在当年中的序号以及该年的总天数
    let (current_day_of_year, total_days) = utils::get_year_info(year, month, day);

    // 计算ISO标准下该周周四在全年的序号
    let diff = 4 - iso_week_day;
    let thursday_day_of_year = current_day_of_year + diff;

    // 如果周四的序号小于1，则说明该日期属于上一年度的最后一周
    if thursday_day_of_year < 1 {
        (last_iso_week_number_of_year(year - 1), iso_week_day)
    }
    // 如果周四的序号超出当年的总天数，则说明该日期属于下一年度的第一周
    else if thursday_day_of_year > total_days {
        (1, iso_week_day)
    }
    // 否则，根据周四所在的序号计算出当前周数
    else {
        ((thursday_day_of_year - 1) / 7 + 1, iso_week_day)
    }
}
