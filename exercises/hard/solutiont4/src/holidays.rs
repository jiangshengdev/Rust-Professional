use crate::date;
use crate::date::{Date, DateRange};

/// 表示一个节假日，包含名称、节假日区间和调休上班日期
#[derive(Debug)]
pub struct Holiday {
    /// 节假日名称
    pub name: &'static str,
    /// 节假日区间
    pub period: DateRange,
    /// 调休上班日期
    pub adjusted_workdays: &'static [Date],
}

/// 全局节假日数据
pub static HOLIDAYS: &[Holiday] = &[
    // 一、元旦：1月1日（周三）放假1天，不调休。
    // 定义元旦节，只包含单一天作为休息日
    Holiday {
        name: "元旦",
        period: DateRange {
            start: Date { month: 1, day: 1 },
            end: Date { month: 1, day: 1 },
        },
        adjusted_workdays: &[],
    },
    // 二、春节：1月28日（农历除夕、周二）至2月4日（农历正月初七、周二）放假调休，共8天。1月26日（周日）、2月8日（周六）上班。
    // 定义春节，节假日区间为1月28日到2月4日，并且包含了调休上班日期
    Holiday {
        name: "春节",
        period: DateRange {
            start: Date { month: 1, day: 28 },
            end: Date { month: 2, day: 4 },
        },
        adjusted_workdays: &[Date { month: 1, day: 26 }, Date { month: 2, day: 8 }],
    },
    // 三、清明节：4月4日（周五）至6日（周日）放假，共3天。
    // 定义清明节，区间为4月4日至4月6日，没有调休上班日期
    Holiday {
        name: "清明节",
        period: DateRange {
            start: Date { month: 4, day: 4 },
            end: Date { month: 4, day: 6 },
        },
        adjusted_workdays: &[],
    },
    // 四、劳动节：5月1日（周四）至5日（周一）放假调休，共5天。4月27日（周日）上班。
    // 定义劳动节，区间为5月1日至5月5日，并且有调休上班日期（4月27日）
    Holiday {
        name: "劳动节",
        period: DateRange {
            start: Date { month: 5, day: 1 },
            end: Date { month: 5, day: 5 },
        },
        adjusted_workdays: &[Date { month: 4, day: 27 }],
    },
    // 五、端午节：5月31日（周六）至6月2日（周一）放假，共3天。
    // 定义端午节，节假日区间为5月31日至6月2日，没有调休上班日期
    Holiday {
        name: "端午节",
        period: DateRange {
            start: Date { month: 5, day: 31 },
            end: Date { month: 6, day: 2 },
        },
        adjusted_workdays: &[],
    },
    // 六、国庆节、中秋节：10月1日（周三）至8日（周三）放假调休，共8天。9月28日（周日）、10月11日（周六）上班。
    // 定义国庆节与中秋节，区间为10月1日至10月8日，同时包含了两个调休上班日期
    Holiday {
        name: "国庆节、中秋节",
        period: DateRange {
            start: Date { month: 10, day: 1 },
            end: Date { month: 10, day: 8 },
        },
        adjusted_workdays: &[Date { month: 9, day: 28 }, Date { month: 10, day: 11 }],
    },
];

/// 根据指定日期查询是否处于节假日中，并返回匹配的节假日。
///
/// # 参数
/// - `date`: 要查询的日期，类型为 [`Date`]
///
/// # 返回值
/// 返回匹配的节假日的静态引用的 [`Holiday`]，如果没有匹配到则返回 [`None`]
pub fn query_holiday(date: &Date) -> Option<&'static Holiday> {
    // 遍历所有节假日数据，查找满足日期在主要节假日区间内的节假日
    HOLIDAYS.iter().find(|&holiday| {
        // 检查给定日期是否处于当前节假日的区间内
        date::date_in_range(date, &holiday.period)
    })
}
