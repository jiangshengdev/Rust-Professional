use crate::date::Date;
use crate::{holidays, utils};

/// 判断指定日期是否为交易日，即不是节假日且不为周末
///
/// # 参数
///
/// * `year` - 年份，例如 2025
/// * `month` - 月份（1 到 12）
/// * `day` - 日
///
/// # 返回值
///
/// 返回 `true` 表示该日期为交易日，否则返回 `false`
fn is_trading_day(year: i32, month: usize, day: i32) -> bool {
    let date = Date::new(month as u32, day as u32);
    holidays::query_holiday(&date).is_none() && !utils::is_weekend(year, month, day)
}

/// 计算当前日期到下一个交易日的剩余天数
///
/// 注意：该函数从当前日期的下一天开始计算，直至找到第一个交易日
///
/// # 参数
///
/// * `year` - 当前日期所在的年份
/// * `month` - 当前日期所在的月份（1 到 12）
/// * `day` - 当前日期
///
/// # 返回值
///
/// 返回从当前日期的下一天起到下一个交易日之间的天数
pub fn days_until_a_share(year: i32, month: usize, day: i32) -> i32 {
    let (mut y, mut m, mut d) = utils::next_day(year, month, day);
    let mut count = 0;
    while !is_trading_day(y, m, d) {
        let (ny, nm, nd) = utils::next_day(y, m, d);
        y = ny;
        m = nm;
        d = nd;
        count += 1;
    }
    count
}
