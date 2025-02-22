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
/// * `u32` - 年份
/// * `u32` - 月份
/// * `u32` - 日份
pub fn parse_date(time: &str) -> (u32, u32, u32) {
    // 将输入的日期字符串按照 '-' 分割为各个部分
    let mut parts = time.split('-');

    // 从分割结果中获取年份字符串，并解析为 u32 类型
    let year = parts.next().unwrap().parse().unwrap();

    // 从分割结果中获取月份字符串，并解析为 u32 类型
    let month = parts.next().unwrap().parse().unwrap();

    // 从分割结果中获取日份字符串，并解析为 u32 类型
    let day = parts.next().unwrap().parse().unwrap();

    // 返回解析后的年份、月份和日份组成的元组
    (year, month, day)
}
