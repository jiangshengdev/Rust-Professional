/// 解析日期字符串
///
/// # 参数
///
/// * `time` - 格式为 "年-月-日" 的字符串
///
/// # 返回值
///
/// 返回一个元组，包含：
///
/// * `u32` - 年
/// * `u32` - 月
/// * `u32` - 日
pub fn parse_date(time: &str) -> (u32, u32, u32) {
    // 按照横线拆分字符串
    let mut parts = time.split('-');

    // 解析年
    let year = parts.next().unwrap().parse().unwrap();

    // 解析月
    let month = parts.next().unwrap().parse().unwrap();

    // 解析日
    let day = parts.next().unwrap().parse().unwrap();

    // 返回年月日元组
    (year, month, day)
}
