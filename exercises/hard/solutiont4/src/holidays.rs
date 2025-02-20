use crate::date;
use crate::date::{Date, DateRange};

/// 表示一个节假日，包含名称、节假日区间和额外休息日期
#[derive(Debug)]
pub struct Holiday {
    /// 节假日名称
    pub name: &'static str,
    /// 节假日区间
    pub period: DateRange,
    /// 额外休息日期
    pub extra: &'static [Date],
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

/// 根据指定日期查询是否处于节假日中，并返回匹配的节假日。
///
/// # 参数
/// - `date`: 要查询的日期，类型为 [`Date`]
///
/// # 返回值
/// 返回匹配的节假日的静态引用的 [`Holiday`]，如果没有匹配到则返回 [`None`]
///
pub fn query_holiday(date: &Date) -> Option<&'static Holiday> {
    HOLIDAYS
        .iter()
        .find(|&holiday| date::date_in_range(date, &holiday.period) || holiday.extra.contains(date))
}
