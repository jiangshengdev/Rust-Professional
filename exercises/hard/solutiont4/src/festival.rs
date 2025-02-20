use crate::utils;

/// 根据年份返回节日的日期
///
/// # 参数
/// - `year`: 输入的年份
///
/// # 返回
/// 返回一个元组 `(festival_month, festival_day)`，
/// 其中 `festival_month` 是节日的月份 (usize)，
/// `festival_day` 是节日的日期 (i32)。
fn get_festival_date(year: i32) -> (usize, i32) {
    match year {
        2025 => (1, 29),
        2026 => (2, 17),
        _ => (1, 31),
    }
}

/// 计算当前日期到节日的剩余天数
///
/// # 参数
/// - `year`: 当前年份
/// - `month`: 当前月份（从1开始）
/// - `day`: 当前的日期
///
/// # 返回
/// 返回从当前日期到节日的剩余天数。<br>
/// 若节日在当前日期之前，则返回到下一年节日的剩余天数。
pub fn days_until_festival(year: i32, month: usize, day: i32) -> i32 {
    let (festival_month, festival_day) = get_festival_date(year);
    let current_day_of_year = utils::day_of_year(year, month, day);
    let festival_day_of_year = utils::day_of_year(year, festival_month, festival_day);
    let this_year_left = utils::days_in_year(year) - current_day_of_year;

    if festival_day_of_year <= current_day_of_year {
        let next_year = year + 1;
        let (next_fm, next_fd) = get_festival_date(next_year);
        let next_festival_day_of_year = utils::day_of_year(next_year, next_fm, next_fd);
        this_year_left + next_festival_day_of_year
    } else {
        festival_day_of_year - current_day_of_year
    }
}
