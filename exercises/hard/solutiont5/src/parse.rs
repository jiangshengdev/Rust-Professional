/// 解析出生年月字符串，返回出生年份和月份
///
/// # 参数
/// * `time` - 出生年月字符串，格式 "YYYY-MM"
///
/// # 返回
/// 返回一个元组 `(birth_year, birth_month)`
/// * `birth_year` - 出生年份
/// * `birth_month` - 出生月份
pub fn parse_birth_date(time: &str) -> (i32, i32) {
    // 根据 '-' 分割输入字符串
    let parts: Vec<&str> = time.split('-').collect();
    let birth_year: i32 = parts[0].parse().unwrap();
    let birth_month: i32 = parts[1].parse().unwrap();
    (birth_year, birth_month)
}
