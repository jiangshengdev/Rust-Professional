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

/// 表示一个节假日，包含名称、节假日区间和额外休息日期
#[derive(Debug)]
pub struct Holiday {
    pub name: &'static str,
    pub period: DateRange,
    pub extra: &'static [Date],
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

/// 全局节假日数据
pub static HOLIDAYS: &[Holiday] = &[
    Holiday {
        name: "元旦",
        period: DateRange {
            start: Date { month: 1, day: 1 },
            end: Date { month: 1, day: 1 },
        },
        extra: &[],
    },
    Holiday {
        name: "春节",
        period: DateRange {
            start: Date { month: 1, day: 28 },
            end: Date { month: 2, day: 4 },
        },
        extra: &[Date { month: 1, day: 26 }, Date { month: 2, day: 8 }],
    },
    Holiday {
        name: "清明节",
        period: DateRange {
            start: Date { month: 4, day: 4 },
            end: Date { month: 4, day: 6 },
        },
        extra: &[],
    },
    Holiday {
        name: "劳动节",
        period: DateRange {
            start: Date { month: 5, day: 1 },
            end: Date { month: 5, day: 5 },
        },
        extra: &[Date { month: 4, day: 27 }],
    },
    Holiday {
        name: "端午节",
        period: DateRange {
            start: Date { month: 5, day: 31 },
            end: Date { month: 6, day: 2 },
        },
        extra: &[],
    },
    Holiday {
        name: "国庆节、中秋节",
        period: DateRange {
            start: Date { month: 10, day: 1 },
            end: Date { month: 10, day: 8 },
        },
        extra: &[Date { month: 9, day: 28 }, Date { month: 10, day: 11 }],
    },
];

/// 根据指定日期查询是否处于节假日中，返回匹配的节假日
pub fn query_holiday(date: &Date) -> Option<&'static Holiday> {
    HOLIDAYS
        .iter()
        .find(|&holiday| date_in_range(date, &holiday.period) || holiday.extra.contains(date))
}

/// 判断是否为闰年
fn is_leap_year(year: i32) -> bool {
    (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0)
}

/// 计算每月的天数，闰年第二个月返回29天
fn compute_days_in_month(year: i32) -> [i32; 12] {
    let mut days_in_month = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
    if is_leap_year(year) {
        days_in_month[1] = 29;
    }
    days_in_month
}

/// 获取当前是年份中的第几天，以及当年总天数
fn get_year_info(year: i32, month: usize, day: i32) -> (i32, i32) {
    let days_in_month = compute_days_in_month(year);
    let current_day: i32 = days_in_month.iter().take(month - 1).sum::<i32>() + day;
    let total_days: i32 = days_in_month.iter().sum();
    (current_day, total_days)
}

/// 获取日期对应的农历中的第几天
fn day_of_year(year: i32, month: usize, day: i32) -> i32 {
    get_year_info(year, month, day).0
}

/// 计算指定年份的总天数
fn days_in_year(year: i32) -> i32 {
    compute_days_in_month(year).iter().sum()
}

/// 使用坂本算法计算星期几
fn day_of_week_sakamoto(year: i32, month: usize, day: i32) -> i32 {
    let mut y = year;
    let mut m = month as i32;
    if m == 1 || m == 2 {
        y -= 1;
        m += 12;
    }
    let k = y % 100;
    let j = y / 100;
    let h = (day + (13 * (m + 1)) / 5 + k + (k / 4) + (j / 4) + 5 * j) % 7;
    (h + 5) % 7 + 1
}

/// 获取上一年度最后一周的 ISO 周数
fn last_iso_week_number_of_year(year: i32) -> i32 {
    let prev_year_total = if is_leap_year(year) { 366 } else { 365 };
    let prev_dec31_week_day = day_of_week_sakamoto(year, 12, 31);
    let prev_thursday = prev_year_total + (4 - prev_dec31_week_day);
    (prev_thursday - 1) / 7 + 1
}

/// 根据给定日期计算 ISO 周数和周几
fn iso_week_number(year: i32, month: usize, day: i32) -> (i32, i32) {
    let iso_week_day = day_of_week_sakamoto(year, month, day);
    let (current_day_of_year, total_days) = get_year_info(year, month, day);
    let diff = 4 - iso_week_day;
    let thursday_day_of_year = current_day_of_year + diff;
    if thursday_day_of_year < 1 {
        (last_iso_week_number_of_year(year - 1), iso_week_day)
    } else if thursday_day_of_year > total_days {
        (1, iso_week_day)
    } else {
        ((thursday_day_of_year - 1) / 7 + 1, iso_week_day)
    }
}

/// 根据年份返回节日的日期
fn get_festival_date(year: i32) -> (usize, i32) {
    match year {
        2025 => (1, 29),
        2026 => (2, 17),
        _ => (1, 31),
    }
}

/// 计算当前日期到节日的剩余天数
fn days_until_festival(year: i32, month: usize, day: i32) -> i32 {
    let (festival_month, festival_day) = get_festival_date(year);
    let current_day_of_year = day_of_year(year, month, day);
    let festival_day_of_year = day_of_year(year, festival_month, festival_day);
    let this_year_left = days_in_year(year) - current_day_of_year;

    if festival_day_of_year <= current_day_of_year {
        let next_year = year + 1;
        let (next_fm, next_fd) = get_festival_date(next_year);
        let next_festival_day_of_year = day_of_year(next_year, next_fm, next_fd);
        this_year_left + next_festival_day_of_year
    } else {
        festival_day_of_year - current_day_of_year
    }
}

/// 获取指定日期的下一天
fn next_day(year: i32, month: usize, day: i32) -> (i32, usize, i32) {
    let days_in_month = compute_days_in_month(year);
    if day < days_in_month[month - 1] {
        (year, month, day + 1)
    } else if month < 12 {
        (year, month + 1, 1)
    } else {
        (year + 1, 1, 1)
    }
}

/// 判断指定日期是否为周末
fn is_weekend(year: i32, month: usize, day: i32) -> bool {
    let dow = day_of_week_sakamoto(year, month, day);
    dow == 6 || dow == 7
}

/// 判断指定日期是否为交易日，不是节假日且不为周末
fn is_trading_day(year: i32, month: usize, day: i32) -> bool {
    let date = Date::new(month as u32, day as u32);
    query_holiday(&date).is_none() && !is_weekend(year, month, day)
}

/// 计算当前日期到下一个交易日的剩余天数
fn days_until_ashare(year: i32, month: usize, day: i32) -> i32 {
    let (mut y, mut m, mut d) = next_day(year, month, day);
    let mut count = 0;
    while !is_trading_day(y, m, d) {
        let (ny, nm, nd) = next_day(y, m, d);
        y = ny;
        m = nm;
        d = nd;
        count += 1;
    }
    count
}

/// 解析日期字符串，格式为 "年-月-日"
fn parse_date(time: &str) -> (i32, usize, i32) {
    let mut parts = time.split('-');
    let year = parts.next().unwrap().parse().unwrap();
    let month = parts.next().unwrap().parse().unwrap();
    let day = parts.next().unwrap().parse().unwrap();
    (year, month, day)
}

/// 根据输入时间字符串，返回 ISO 周数、周几、当前是年中第几天、年剩余天数、到节日与下个交易日剩余天数
pub fn time_info(time: &str) -> String {
    let (year, month, day) = parse_date(time);

    let (iso_week_number, iso_week_day) = iso_week_number(year, month, day);
    let (doy, total_days) = get_year_info(year, month, day);
    let days_left_in_year = total_days - doy;
    let festival_left = days_until_festival(year, month, day);
    let ashare_left = days_until_ashare(year, month, day);

    format!(
        "{},{},{},{},{},{}",
        iso_week_number, iso_week_day, doy, days_left_in_year, festival_left, ashare_left
    )
}
