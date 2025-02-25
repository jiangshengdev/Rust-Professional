/// 根据工种获取对应的退休参数
///
/// # 参数
/// * `tp` - 工种类型
///
/// # 返回
/// 返回一个元组 `(base_age, delay_interval, target_age)`
/// * `base_age` - 基础退休年龄
/// * `delay_interval` - 延迟间隔参数，用于计算延迟月数
/// * `target_age` - 目标退休年龄
pub fn get_retirement_params(tp: &str) -> (i32, i32, i32) {
    // 匹配工种，返回相应退休参数
    match tp {
        "男职工" => (60, 4, 63),
        "原法定退休年龄55周岁女职工" => (55, 4, 58),
        "原法定退休年龄50周岁女职工" => (50, 2, 55),
        _ => panic!("Invalid worker type"),
    }
}
