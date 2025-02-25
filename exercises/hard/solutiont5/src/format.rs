/// 格式化年龄，去除多余的小数部分
///
/// # 参数
/// * `age` - 实际年龄（浮点数）
///
/// # 返回
/// 格式化后的年龄字符串
pub fn format_age(age: f64) -> String {
    // 按两位小数格式化年龄
    let age_str = format!("{:.2}", age);

    if age_str.ends_with(".00") {
        // 若小数部分为 .00，则移除小数部分
        return age_str[..age_str.len() - 3].to_string();
    }

    if age_str.ends_with('0') {
        // 若末尾为多余的0，则移除该字符
        return age_str[..age_str.len() - 1].to_string();
    }

    age_str
}
