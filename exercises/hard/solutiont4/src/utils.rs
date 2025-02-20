use crate::iso_week;

/// 判断是否为闰年
///
/// # 参数
/// - `year`: 要判断的年份
///
/// # 返回值
/// - `true` 如果是闰年，否则返回 `false`
pub fn is_leap_year(year: i32) -> bool {
    // 判断年份是否满足闰年的条件
    (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0)
}

/// 计算每月的天数，闰年第二个月返回29天
///
/// # 参数
/// - `year`: 指定的年份
///
/// # 返回值
/// - 返回一个包含每月天数的数组，若是闰年，则二月为29天
pub fn compute_days_in_month(year: i32) -> [i32; 12] {
    // 定义非闰年每个月的天数
    let mut days_in_month = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
    // 如果是闰年则修改2月天数为29天
    if is_leap_year(year) {
        days_in_month[1] = 29;
    }
    days_in_month
}

/// 获取当前是年份中的第几天，以及当年总天数
///
/// # 参数
/// - `year`: 指定的年份
/// - `month`: 指定的月份 (1-12)
/// - `day`: 指定的日期
///
/// # 返回值
/// - `(current_day, total_days)`
///   - `current_day`: 指定日期是当年的第几天
///   - `total_days`: 当年的总天数（365或366）
pub fn get_year_info(year: i32, month: usize, day: i32) -> (i32, i32) {
    // 先获取每个月的天数数组
    let days_in_month = compute_days_in_month(year);
    // 累加前几个月的天数，再加上当前月份的日期，得到当前天数
    let current_day: i32 = days_in_month.iter().take(month - 1).sum::<i32>() + day;
    // 全年总天数为各个月天数之和
    let total_days: i32 = days_in_month.iter().sum();
    (current_day, total_days)
}

/// 获取日期对应的农历中的第几天
///
/// # 参数
/// - `year`: 指定的年份
/// - `month`: 指定的月份 (1-12)
/// - `day`: 指定的日期
///
/// # 返回值
/// - 返回该日期是当年的第几天
pub fn day_of_year(year: i32, month: usize, day: i32) -> i32 {
    // 获取当前天数（当年的第几天）
    get_year_info(year, month, day).0
}

/// 计算指定年份的总天数
///
/// # 参数
/// - `year`: 指定的年份
///
/// # 返回值
/// - 返回该年份的总天数（365或366）
pub fn days_in_year(year: i32) -> i32 {
    // 获取每个月的天数并求和，得出全年总天数
    compute_days_in_month(year).iter().sum()
}

/// 获取指定日期的下一天
///
/// # 参数
/// - `year`: 当前日期的年份
/// - `month`: 当前日期的月份 (1-12)
/// - `day`: 当前日期的日
///
/// # 返回值
/// - `(next_year, next_month, next_day)` 表示下一天的日期
pub fn next_day(year: i32, month: usize, day: i32) -> (i32, usize, i32) {
    // 获取当前年份中每个月的天数
    let days_in_month = compute_days_in_month(year);
    // 如果当前日期未达到当月最大天数，则次日只需要增加一天
    if day < days_in_month[month - 1] {
        (year, month, day + 1)
    // 如果不是12月份，则进入下个月，并将日期设为1号
    } else if month < 12 {
        (year, month + 1, 1)
    // 如果当前为12月最后一天，则进入下一年的1月1号
    } else {
        (year + 1, 1, 1)
    }
}

/// 判断指定日期是否为周末
///
/// # 参数
/// - `year`: 指定的年份
/// - `month`: 指定的月份 (1-12)
/// - `day`: 指定的日期
///
/// # 返回值
/// - `true` 如果该日期为周六或周日，否则返回 `false`
pub fn is_weekend(year: i32, month: usize, day: i32) -> bool {
    // 获取日期对应的星期，返回1至7对应周一到周日
    let dow = iso_week::day_of_week_sakamoto(year, month, day);
    // 判断星期是否为6（周六）或7（周日）
    dow == 6 || dow == 7
}
