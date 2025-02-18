use std::collections::{HashMap, HashSet};
use std::fs;

/// 从文件中读取数据并计算各批次的连通省份数，返回用逗号分隔的字符串
pub fn count_provinces() -> String {
    // 读取存储数据的文件内容
    let content = match fs::read_to_string("./district.json") {
        Ok(s) => s,
        Err(e) => {
            // 读取失败则直接返回空字符串
            eprintln!("读取文件失败: {}", e);
            return "".to_string();
        }
    };

    // 过滤掉以 // 开头的注释行
    let filtered = content
        .lines()
        .filter(|line| !line.trim_start().starts_with("//"))
        .collect::<Vec<_>>()
        .join("\n");

    // 自定义解析非规范 JSON 文件（只解析外层对象与内层城市映射）
    let batches = match parse_batches(&filtered) {
        Ok(b) => b,
        Err(e) => {
            eprintln!("解析文件失败: {}", e);
            return "".to_string();
        }
    };

    // 依次计算每个批次的连通分量数
    // 这里根据批次的 key 经过数字排序得到顺序
    let mut keys: Vec<_> = batches.keys().collect();
    keys.sort_by_key(|k| k.parse::<i32>().unwrap_or(0));

    let mut results = Vec::new();
    for key in keys {
        // 获取当前批次
        let batch = &batches[key];
        // 构造无向图，key为城市，value为关联的城市集合
        let mut graph: HashMap<String, Vec<String>> = HashMap::new();
        // 遍历城市映射数据
        for (city, neighbors) in batch {
            // 将 city 与其邻接城市建立双向边关系
            graph.entry(city.clone()).or_default();
            for n in neighbors {
                graph.entry(n.clone()).or_default();
            }
            // 添加正向边
            graph
                .get_mut(city)
                .unwrap()
                .extend(neighbors.iter().cloned());
            // 添加反向边
            for n in neighbors {
                graph.get_mut(n).unwrap().push(city.clone());
            }
        }

        // 计算无向图的连通分量
        let mut visited: HashSet<String> = HashSet::new();
        let mut count = 0;
        for node in graph.keys() {
            if !visited.contains(node) {
                // 进行深度优先搜索
                dfs(node, &graph, &mut visited);
                count += 1;
            }
        }
        results.push(count.to_string());
    }

    // 将每个批次的结果用逗号分隔后返回
    results.join(",")
}

/// 深度优先搜索遍历无向图
fn dfs(node: &str, graph: &HashMap<String, Vec<String>>, visited: &mut HashSet<String>) {
    // 标记当前节点已访问
    visited.insert(node.to_string());
    // 遍历当前节点的所有邻居
    if let Some(neighbors) = graph.get(node) {
        for neighbor in neighbors {
            if !visited.contains(neighbor) {
                dfs(neighbor, graph, visited);
            }
        }
    }
}

/// 批次数据的类型，每个批次为：城市名称 -> 关联的城市列表
type Batch = HashMap<String, Vec<String>>;
/// 所有批次数据类型：批次键 -> 批次内容
type Batches = HashMap<String, Batch>;

/// 解析非规范的 district.json 文本数据，返回所有批次数据
fn parse_batches(s: &str) -> Result<Batches, String> {
    let s = s.trim();
    // 检查外层是否以大括号包围
    if !s.starts_with('{') || !s.ends_with('}') {
        return Err("外层缺少大括号".to_string());
    }
    // 去除外层的大括号
    let inner = &s[1..s.len() - 1];
    let mut batches: Batches = HashMap::new();
    let mut pos = 0;
    let chars: Vec<char> = inner.chars().collect();

    while pos < chars.len() {
        skip_whitespace(&chars, &mut pos);
        if pos >= chars.len() {
            break;
        }
        // 读取批次 key（要求为字符串，可能为数字字符串）
        let key = parse_string(&chars, &mut pos)?;
        skip_whitespace(&chars, &mut pos);
        // 跳过冒号
        if pos >= chars.len() || chars[pos] != ':' {
            return Err("冒号缺失".to_string());
        }
        pos += 1;
        skip_whitespace(&chars, &mut pos);
        // 下一个值应为内层对象
        let batch_obj = parse_inner_object(&chars, &mut pos)?;
        // 若已存在相同 key，则合并当前 batch 对象
        batches
            .entry(key)
            .and_modify(|b: &mut Batch| merge_batches(b, &batch_obj))
            .or_insert(batch_obj);
        skip_whitespace(&chars, &mut pos);
        // 如果下一个字符是逗号，则跳过
        if pos < chars.len() && chars[pos] == ',' {
            pos += 1;
        }
    }
    Ok(batches)
}

/// 合并两个批次对象，若存在相同城市，合并关联城市列表
fn merge_batches(target: &mut Batch, other: &Batch) {
    for (city, neighbors) in other {
        target
            .entry(city.clone())
            .and_modify(|v| {
                // 添加时去重
                for n in neighbors {
                    if !v.contains(n) {
                        v.push(n.clone());
                    }
                }
            })
            .or_insert(neighbors.clone());
    }
}

/// 解析内层对象，返回城市 -> 关联城市列表
fn parse_inner_object(chars: &[char], pos: &mut usize) -> Result<Batch, String> {
    let mut map = HashMap::new();
    skip_whitespace(chars, pos);
    if *pos >= chars.len() || chars[*pos] != '{' {
        return Err("内层对象未以 { 开始".to_string());
    }
    *pos += 1;
    loop {
        skip_whitespace(chars, pos);
        if *pos >= chars.len() {
            return Err("内层对象未正确关闭".to_string());
        }
        if chars[*pos] == '}' {
            *pos += 1;
            break;
        }
        // 解析城市名称
        let city = parse_string(chars, pos)?;
        skip_whitespace(chars, pos);
        if *pos >= chars.len() || chars[*pos] != ':' {
            return Err("内层对象中冒号缺失".to_string());
        }
        *pos += 1;
        skip_whitespace(chars, pos);
        // 解析关联城市数组
        let arr = parse_array(chars, pos)?;
        // 如出现重复城市则合并数组
        map.entry(city)
            .and_modify(|v: &mut Vec<String>| {
                for s in &arr {
                    if !v.contains(s) {
                        v.push(s.clone());
                    }
                }
            })
            .or_insert(arr);
        skip_whitespace(chars, pos);
        // 如果下一个字符为逗号，则跳过
        if *pos < chars.len() && chars[*pos] == ',' {
            *pos += 1;
        }
    }
    Ok(map)
}

/// 解析数组，返回字符串向量
fn parse_array(chars: &[char], pos: &mut usize) -> Result<Vec<String>, String> {
    let mut arr = Vec::new();
    skip_whitespace(chars, pos);
    if *pos >= chars.len() || chars[*pos] != '[' {
        return Err("数组未以 [ 开始".to_string());
    }
    *pos += 1;
    loop {
        skip_whitespace(chars, pos);
        if *pos >= chars.len() {
            return Err("数组未正确结束".to_string());
        }
        if chars[*pos] == ']' {
            *pos += 1;
            break;
        }
        // 解析数组内的字符串
        let s_val = parse_string(chars, pos)?;
        arr.push(s_val);
        skip_whitespace(chars, pos);
        if *pos < chars.len() && chars[*pos] == ',' {
            *pos += 1;
        }
    }
    Ok(arr)
}

/// 解析一个字符串（要求以双引号包围），返回内部字符内容
fn parse_string(chars: &[char], pos: &mut usize) -> Result<String, String> {
    skip_whitespace(chars, pos);
    if *pos >= chars.len() || chars[*pos] != '"' {
        return Err("字符串未以 \" 开始".to_string());
    }
    *pos += 1;
    let start = *pos;
    while *pos < chars.len() && chars[*pos] != '"' {
        *pos += 1;
    }
    if *pos >= chars.len() {
        return Err("字符串未正确结束".to_string());
    }
    let s_val: String = chars[start..*pos].iter().collect();
    *pos += 1;
    Ok(s_val)
}

/// 跳过空白字符，包括空格、换行等
fn skip_whitespace(chars: &[char], pos: &mut usize) {
    while *pos < chars.len() && chars[*pos].is_whitespace() {
        *pos += 1;
    }
}
