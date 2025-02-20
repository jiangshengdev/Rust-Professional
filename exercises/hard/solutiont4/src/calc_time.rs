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
    // 调用 parse_date 模块的函数将输入字符串解析为年、月、日
    let (year, month, day) = parse_date::parse_date(time);

    // 利用 iso_week 模块计算 ISO 周编号和星期几
    let (iso_week_number, iso_week_day) = iso_week::iso_week_number(year, month, day);

    // 获取当前日期在年中的天数（doy）以及全年的总天数
    let (doy, total_days) = utils::get_year_info(year, month, day);
    // 计算当前日期距离年末剩余的天数
    let days_left_in_year = total_days - doy;

    // 计算从当前日期到下一个节日的剩余天数
    let festival_left = festival::days_until_festival(year, month, day);

    // 计算从当前日期到下一个交易日的剩余天数
    let ashare_left = a_share::days_until_a_share(year, month, day);

    // 格式化所有计算结果，并以逗号分隔返回一个字符串
    format!(
        "{},{},{},{},{},{}",
        iso_week_number, iso_week_day, doy, days_left_in_year, festival_left, ashare_left
    )
}
