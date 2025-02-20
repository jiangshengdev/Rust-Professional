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
    /// 根据月和日创建新的 Date 实例
    pub fn new(month: u32, day: u32) -> Self {
        Self { month, day }
    }
}

/// 判断指定日期是否在给定的日期范围内
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
