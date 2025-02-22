use crate::{a_share, festival, iso_week, parse_date, utils};

/// 根据输入时间字符串，返回 ISO 周数、周几、当前是年中第几天、年剩余天数、到节日与下个交易日剩余天数
///
/// # 参数
/// - `time`: 格式化时间字符串，格式应符合要求（如 "YYYY-MM-DD"）
///
/// # 返回值
/// 返回包含多个信息的格式化字符串，格式为：
/// "iso_week_number,iso_week_day,doy,days_left_in_year,festival_left,a_share_left"
///
/// # 说明
/// - `iso_week_number`: ISO 周编号
/// - `iso_week_day`: ISO 周内的星期几
/// - `doy`: 当前日期在年内的序号，即今年第几天
/// - `days_left_in_year`: 当年剩余天数
/// - `festival_left`: 离春节（正月初一）剩余天数
/// - `a_share_left`: 离下一次A股开盘日剩余天数
pub fn time_info(time: &str) -> String {
    // 解析输入的日期字符串到年、月、日
    let (year, month, day) = parse_date::parse_date(time);

    // 根据 ISO 规则计算 ISO 周编号和 ISO 周内的星期几
    let (iso_week_number, iso_week_day) = iso_week::iso_week_number(year, month, day);

    // 计算当前日期在年内的序号(doy)和该年的总天数
    let (doy, total_days) = utils::get_year_info(year, month, day);

    // 计算从当前日期起到年底的剩余天数
    let days_left_in_year = total_days - doy;

    // 计算从当前日期到春节(正月初一)的剩余天数，不包含当天
    let festival_left = festival::days_until_festival(year, month, day);

    // 计算从当前日期到下一个A股开盘日的剩余天数，不包含当天
    let a_share_left = a_share::days_until_a_share(year, month, day);

    // 返回按照参数顺序拼接后的计算结果
    format!(
        "{},{},{},{},{},{}",
        iso_week_number, iso_week_day, doy, days_left_in_year, festival_left, a_share_left
    )
}
