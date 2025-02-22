/// 表示一个日期，仅包含月和日
#[derive(Debug, PartialEq, Eq)]
pub struct Date {
    // 月份
    pub month: u32,
    // 日
    pub day: u32,
}

/// 表示一个日期范围，区间为闭区间 [start, end]，包含起始和结束日期
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
    /// 返回包含指定 `month` 和 `day` 的 `Date` 实例
    pub fn new(month: u32, day: u32) -> Self {
        Self { month, day }
    }
}

/// 判断指定日期是否在给定的日期范围内
///
/// # 参数
///
/// * `date` - 需要判断的日期引用
/// * `range` - 日期范围引用，区间为闭区间 [start, end]
///
/// # 返回值
///
/// 如果 `date` 在闭区间 [start, end] 内，则返回 `true`；否则返回 `false`
pub fn date_in_range(date: &Date, range: &DateRange) -> bool {
    // 解构获取待判断日期的月份和日
    let (m, d) = (date.month, date.day);
    // 解构获取日期范围中起始和结束的月份和日
    let (start_m, start_d) = (range.start.month, range.start.day);
    let (end_m, end_d) = (range.end.month, range.end.day);

    // 判断当前月份是否在范围之外
    if m < start_m || m > end_m {
        // 月份在范围外，直接返回 false
        return false;
    }

    if m == start_m {
        // 在起始月份内，判断日期是否早于起始日
        if d < start_d {
            // 日期早于起始日，返回 false
            return false;
        }
    }

    if m == end_m {
        // 在结束月份内，判断日期是否晚于结束日
        if d > end_d {
            // 日期晚于结束日，返回 false
            return false;
        }
    }

    // 如果上述条件均不成立，则日期位于闭区间 [start, end] 内
    true
}
