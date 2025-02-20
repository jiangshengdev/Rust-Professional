/// 解析日期字符串，格式为 "年-月-日"
///
/// # 参数
///
/// * `time` - 包含日期字符串，格式为 "年-月-日"
///
/// # 返回值
///
/// 返回一个元组，其中包含：
///
/// * `i32` - 年份
/// * `usize` - 月份
/// * `i32` - 日份
pub fn parse_date(time: &str) -> (i32, usize, i32) {
    let mut parts = time.split('-');
    let year = parts.next().unwrap().parse().unwrap();
    let month = parts.next().unwrap().parse().unwrap();
    let day = parts.next().unwrap().parse().unwrap();
    (year, month, day)
}
