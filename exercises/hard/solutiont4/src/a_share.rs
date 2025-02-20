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
    // 构造日期对象，便于后续进行节假日查询
    let date = Date::new(month as u32, day as u32);

    // 首先判断指定日期是否为节假日
    // 同时判断该日期是否为周末（通过utils::is_weekend函数实现）
    // 如果既不是节假日又不为周末，则视为交易日
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
    // 调用next_day函数获取当前日期的下一天
    let (mut y, mut m, mut d) = utils::next_day(year, month, day);

    // 初始化计数器，用于记录跨越的天数
    let mut count = 0;

    // 当当前日期不是交易日时，持续往后查找
    while !is_trading_day(y, m, d) {
        // 计算下一天的日期
        let (ny, nm, nd) = utils::next_day(y, m, d);

        // 更新日期变量为最新计算的下一天
        y = ny;
        m = nm;
        d = nd;

        // 累计跨越的天数
        count += 1;
    }

    // 返回从当前日期到下一个交易日的累计天数
    count
}
