/// 表示一个日期，仅包含月和日
#[derive(Debug, PartialEq, Eq)]
pub struct Date {
    pub month: u32,
    pub day: u32,
}

/// 表示一个日期范围，包含开始和结束日期
#[derive(Debug)]
pub struct DateRange {
    pub start: Date,
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
    let (m, d) = (date.month, date.day);
    let (start_m, start_d) = (range.start.month, range.start.day);
    let (end_m, end_d) = (range.end.month, range.end.day);

    if m < start_m || m > end_m {
        return false;
    }
    if m == start_m && d < start_d {
        return false;
    }
    if m == end_m && d > end_d {
        return false;
    }
    true
}
