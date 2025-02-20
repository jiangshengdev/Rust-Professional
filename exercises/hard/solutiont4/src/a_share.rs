use crate::date::Date;
use crate::{holidays, utils};

/// 判断指定日期是否为交易日，不是节假日且不为周末
fn is_trading_day(year: i32, month: usize, day: i32) -> bool {
    let date = Date::new(month as u32, day as u32);
    holidays::query_holiday(&date).is_none() && !utils::is_weekend(year, month, day)
}

/// 计算当前日期到下一个交易日的剩余天数
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
