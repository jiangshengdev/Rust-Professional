use crate::utils;

/// 根据年份返回节日的日期
///
/// # 参数
/// - `year`: 输入的年份
///
/// # 返回
/// 返回一个元组 `(festival_month, festival_day)`，
/// 其中 `festival_month` 是节日的月份 (u32)，
/// `festival_day` 是节日的日期 (u32)。
fn get_festival_date(year: u32) -> (u32, u32) {
    // 根据不同年份返回对应的节日日期
    match year {
        2015 => (2, 19),
        2016 => (2, 8),
        2017 => (1, 28),
        2018 => (2, 16),
        2019 => (2, 5),
        2020 => (1, 25),
        2021 => (2, 12),
        2022 => (2, 1),
        2023 => (1, 22),
        2024 => (2, 10),
        2025 => (1, 29),
        2026 => (2, 17),
        2027 => (2, 6),
        2028 => (1, 26),
        2029 => (2, 13),
        2030 => (2, 3),
        2031 => (1, 23),
        2032 => (2, 11),
        2033 => (1, 31),
        2034 => (2, 19),
        2035 => (2, 8),
        // 对于未覆盖的年份，直接 panic
        _ => {
            panic!("年份 {} 不在有效范围内", year)
        }
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
pub fn days_until_festival(year: u32, month: u32, day: u32) -> u32 {
    // 获取今年对应节日的月份和日期
    let (festival_month, festival_day) = get_festival_date(year);

    // 计算当前日期在本年的天数
    let current_day_of_year = utils::day_of_year(year, month, day);

    // 计算今年节日所在的天数位置
    let festival_day_of_year = utils::day_of_year(year, festival_month, festival_day);

    // 计算从当前日期到本年结束的天数
    let this_year_left = utils::days_in_year(year) - current_day_of_year;

    // 判断如果节日已经过了或者正好是当前日期
    if festival_day_of_year <= current_day_of_year {
        // 计算下一年的年份
        let next_year = year + 1;

        // 获取下一年节日的月份和日期
        let (next_fm, next_fd) = get_festival_date(next_year);

        // 计算下一年节日所在的天数位置
        let next_festival_day_of_year = utils::day_of_year(next_year, next_fm, next_fd);

        // 返回当前日期到本年结束的天数与下一年节日天数之和
        this_year_left + next_festival_day_of_year
    } else {
        // 若节日还未到，返回当前与节日之间的天数差
        festival_day_of_year - current_day_of_year
    }
}
