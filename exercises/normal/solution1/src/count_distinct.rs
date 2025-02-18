use std::collections::HashSet;

pub fn new_count_distinct(input_str: &str) -> usize {
    // 按逗号分割字符串得到迭代器
    let parts = input_str.split(',');

    // 创建 HashSet 保存所有不重复的部分
    let mut set = HashSet::new();

    // 遍历每一个分割结果
    for part in parts {
        // 将每个部分插入 HashSet 中
        set.insert(part);
    }

    // 返回 HashSet 中不重复元素的数量
    set.len()
}
