/// 表示一个日期，仅包含月和日
#[derive(Debug, PartialEq, Eq)]
pub struct Date {
    // 月份
    pub month: u32,
    // 日
    pub day: u32,
}

/// 表示一个日期范围，包含开始和结束日期
#[derive(Debug)]
pub struct DateRange {
    // 起始日期
    pub start: Date,
    // 结束日期
    pub end: Date,
}

impl Date {
    /// 创建新的 `Date` 实例
    ///
    /// # 参数
    ///
    /// * `month` - 月份，取值范围为 1 到 12。
    /// * `day` - 日期，基于月份提供相应的天数。
    ///
    /// # 返回值
    ///
    /// 返回一个包含指定 `month` 和 `day` 的 `Date` 实例。
    pub fn new(month: u32, day: u32) -> Self {
        Self { month, day }
    }
}

/// 判断指定日期是否在给定的日期范围内
///
/// # 参数
///
/// * `date` - 需要判断的日期引用。
/// * `range` - 日期范围引用，包含起始和结束日期。
///
/// # 返回值
///
/// 如果 `date` 在 `range` 范围内（包含边界），则返回 `true`，否则返回 `false`。
pub fn date_in_range(date: &Date, range: &DateRange) -> bool {
    // 解构获取待判断日期的月份和日
    let (m, d) = (date.month, date.day);
    // 解构获取日期范围中起始和结束的月份和日
    let (start_m, start_d) = (range.start.month, range.start.day);
    let (end_m, end_d) = (range.end.month, range.end.day);

    // 判断月份是否在范围之外
    if m < start_m || m > end_m {
        return false;
    }
    // 如果在起始月份内，则判断日期是否在起始日期之前
    if m == start_m && d < start_d {
        return false;
    }
    // 如果在结束月份内，则判断日期是否在结束日期之后
    if m == end_m && d > end_d {
        return false;
    }
    // 如果上面条件都不成立，则说明日期位于范围内
    true
}
