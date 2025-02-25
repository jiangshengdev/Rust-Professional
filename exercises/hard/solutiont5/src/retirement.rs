use crate::{calculate, compute, format, params, parse, update};

/// 根据出生年月和工种计算并返回退休信息字符串
///
/// # 参数
/// * `time` - 出生年月字符串，格式 "YYYY-MM"
/// * `tp` - 工种，用于匹配对应的退休参数
///
/// # 返回
/// 返回格式化的退休信息字符串，包含退休日期、实际退休年龄和延迟月数
pub fn retire_time(time: &str, tp: &str) -> String {
    // 解析出生年月
    let (birth_year, birth_month) = parse::parse_birth_date(time);

    // 获取对应工种的退休参数
    let (base_age, delay_interval, target_age) = params::get_retirement_params(tp);

    // 计算初步退休日期
    let (mut retire_year, mut retire_month) =
        calculate::calculate_initial_retirement_date(birth_year, birth_month, base_age);

    if retire_year < 2025 {
        // 若退休年份早于2025，则直接计算实际退休年龄
        let actual_age =
            compute::compute_actual_age(birth_year, birth_month, retire_year, retire_month);

        // 格式化输出退休日期、实际退休年龄和延迟月数（固定为0）
        return format!(
            "{}-{:02},{},0",
            retire_year,
            retire_month,
            format::format_age(actual_age)
        );
    }

    // 退休年份不早于2025时，根据延迟规则更新退休日期
    let (new_year, new_month, delay_months) = update::update_retirement_date(
        retire_year,
        retire_month,
        delay_interval,
        target_age,
        base_age,
    );
    retire_year = new_year;
    retire_month = new_month;

    // 根据更新后的退休日期计算实际退休年龄
    let actual_age =
        compute::compute_actual_age(birth_year, birth_month, retire_year, retire_month);

    // 输出更新后的退休日期、实际退休年龄及延迟月数
    format!(
        "{}-{:02},{},{}",
        retire_year,
        retire_month,
        format::format_age(actual_age),
        delay_months
    )
}
