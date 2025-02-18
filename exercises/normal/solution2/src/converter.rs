pub fn convert_base(num_str: &str, to_base: u32) -> String {
    // 检查目标进制是否在允许范围内
    if !(2..=16).contains(&to_base) {
        panic!("目标进制必须在2到16之间")
    }

    // 查找'('字符获取数字基数界限
    let left_paren = num_str.find('(').expect("输入字符串格式错误，找不到'('");
    // 查找')'字符获取数字基数界限
    let right_paren = num_str.find(')').expect("输入字符串格式错误，找不到')'");

    // 提取表示的数字部分字符串
    let num_part = &num_str[0..left_paren];
    // 提取字符串中的原始进制部分
    let orig_base_str = &num_str[left_paren + 1..right_paren];

    // 解析原始进制字符串为数字
    let orig_base: u32 = orig_base_str.trim().parse().expect("无法解析原始进制");

    // 检查原始进制是否在允许范围内
    if !(2..=16).contains(&orig_base) {
        panic!("原始进制必须在2到16之间")
    }

    // 初始化十进制转换数值
    let mut decimal_value: u32 = 0;
    // 遍历输入数字部分的每个字符
    for ch in num_part.chars() {
        // 将字符转换为对应的数字
        let digit = ch.to_digit(orig_base).expect("数字中包含不合法字符");
        // 将当前数字转化累加到几十进制值中
        decimal_value = decimal_value * orig_base + digit;
    }

    // 处理特殊情况：当值为0时直接返回"0"
    if decimal_value == 0 {
        return "0".to_string();
    }

    // 定义字符映射表，用于目标进制转换
    let bases = "0123456789abcdef";
    let mut result = String::new();

    // 循环计算每一位，直至数字转换完毕
    while decimal_value > 0 {
        // 计算当前位的余数
        let remainder = (decimal_value % to_base) as usize;
        // 利用映射表追加对应字符到结果中
        result.push(bases.chars().nth(remainder).unwrap());
        // 更新十进制数值
        decimal_value /= to_base;
    }

    // 反转结果字符串，以得到正确的数字顺序
    result = result.chars().rev().collect();

    // 返回转换后的目标进制数字字符串
    result
}
