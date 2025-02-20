use crate::{a_share, festival, iso_week, parse_date, utils};

/// 根据输入时间字符串，返回 ISO 周数、周几、当前是年中第几天、年剩余天数、到节日与下个交易日剩余天数
///
/// # 参数
/// * `time` - 格式化时间字符串，格式应符合特定要求（如 "YYYY-MM-DD"）
///
/// # 返回值
/// 返回包含多个信息的格式化字符串，格式为：
/// "iso_week_number,iso_week_day,doy,days_left_in_year,festival_left,ashare_left"
pub fn time_info(time: &str) -> String {
    let (year, month, day) = parse_date::parse_date(time);

    let (iso_week_number, iso_week_day) = iso_week::iso_week_number(year, month, day);
    let (doy, total_days) = utils::get_year_info(year, month, day);
    let days_left_in_year = total_days - doy;
    let festival_left = festival::days_until_festival(year, month, day);
    let ashare_left = a_share::days_until_a_share(year, month, day);

    format!(
        "{},{},{},{},{},{}",
        iso_week_number, iso_week_day, doy, days_left_in_year, festival_left, ashare_left
    )
}
