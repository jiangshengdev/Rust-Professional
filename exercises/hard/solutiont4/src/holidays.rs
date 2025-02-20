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
    // 定义元旦节，只包含单一天作为休息日
    Holiday {
        name: "元旦",
        period: DateRange {
            start: Date { month: 1, day: 1 },
            end: Date { month: 1, day: 1 },
        },
        extra: &[],
    },
    // 定义春节，节假日区间为1月28日到2月4日，并且添加了额外的休息日期
    Holiday {
        name: "春节",
        period: DateRange {
            start: Date { month: 1, day: 28 },
            end: Date { month: 2, day: 4 },
        },
        extra: &[Date { month: 1, day: 26 }, Date { month: 2, day: 8 }],
    },
    // 定义清明节，区间为4月4日至4月6日，没有额外休息日期
    Holiday {
        name: "清明节",
        period: DateRange {
            start: Date { month: 4, day: 4 },
            end: Date { month: 4, day: 6 },
        },
        extra: &[],
    },
    // 定义劳动节，区间为5月1日至5月5日，并且有额外的调休日期（4月27日）
    Holiday {
        name: "劳动节",
        period: DateRange {
            start: Date { month: 5, day: 1 },
            end: Date { month: 5, day: 5 },
        },
        extra: &[Date { month: 4, day: 27 }],
    },
    // 定义端午节，节假日区间为5月31日至6月2日，没有额外休息日期
    Holiday {
        name: "端午节",
        period: DateRange {
            start: Date { month: 5, day: 31 },
            end: Date { month: 6, day: 2 },
        },
        extra: &[],
    },
    // 定义国庆节与中秋节，区间为10月1日至10月8日，同时包含了两个额外的休息日期
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
    // 遍历所有节假日数据，查找满足日期在主要节假日区间内或者在额外休息日期中的节假日
    HOLIDAYS.iter().find(|&holiday|
            // 检查给定日期是否处于当前节假日的区间内
            date::date_in_range(date, &holiday.period)
            ||
            // 检查给定日期是否在节假日的额外休息日期中出现
            holiday.extra.contains(date))
}
