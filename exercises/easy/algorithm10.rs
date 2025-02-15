/*
    graph
    This problem requires you to implement a basic graph functio
*/

use colored::Colorize;
use std::collections::{HashMap, HashSet};
use std::fmt;

#[derive(Debug, Clone)]
pub struct NodeNotInGraph;

impl fmt::Display for NodeNotInGraph {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "accessing a node that is not in the graph")
    }
}

pub struct UndirectedGraph {
    adjacency_table: HashMap<String, Vec<(String, i32)>>,
}

impl Graph for UndirectedGraph {
    fn new() -> UndirectedGraph {
        // 创建无向图
        UndirectedGraph {
            adjacency_table: HashMap::new(),
        }
    }

    fn adjacency_table_mutable(&mut self) -> &mut HashMap<String, Vec<(String, i32)>> {
        // 返回可变引用
        &mut self.adjacency_table
    }

    fn adjacency_table(&self) -> &HashMap<String, Vec<(String, i32)>> {
        // 返回不可变引用
        &self.adjacency_table
    }

    fn add_edge(&mut self, edge: (&str, &str, i32)) {
        // 同步实现无向边
        if !self.contains(edge.0) {
            // 如果节点 a 不存在则创建
            self.add_node(edge.0);
        }
        if !self.contains(edge.1) {
            // 如果节点 b 不存在则创建
            self.add_node(edge.1);
        }
        // 获取可变邻接表
        let map = self.adjacency_table_mutable();
        // 在 a 的邻接向量新增 (b, 权重)
        map.get_mut(edge.0)
            .unwrap()
            .push((edge.1.to_string(), edge.2));
        // 在 b 的邻接向量新增 (a, 权重)
        map.get_mut(edge.1)
            .unwrap()
            .push((edge.0.to_string(), edge.2));
    }
}

impl UndirectedGraph {
    // 修改打印图函数，使用固定宽度单元格和彩色边框绘制表格
    pub fn print_graph(&self) -> () {
        // 获取所有节点，排序确保顺序固定
        let mut nodes: Vec<&String> = self.adjacency_table().keys().collect();
        nodes.sort();
        // 计算单元格宽度，取节点名称长度最大值，至少3个字符宽
        let mut max_width = 0;
        for node in nodes.iter() {
            if node.len() > max_width {
                max_width = node.len();
            }
        }
        let cell_width = if max_width < 3 { 3 } else { max_width };
        // 构造上边框，左上角┌，中间分隔┬，右上角┐，边框用浅灰色
        let mut top_border = String::new();
        top_border.push_str("┌");
        top_border.push_str(&"─".repeat(cell_width));
        for _ in 0..nodes.len() {
            top_border.push_str("┬");
            top_border.push_str(&"─".repeat(cell_width));
        }
        top_border.push_str("┐");
        // 构造表头行，第一列为空，其余为节点名称（列头用绿色居中），使用蓝色竖线作为分隔符
        let mut header = String::new();
        header.push_str(&"│".blue().to_string());
        header.push_str(&format!("{:^width$}", "", width = cell_width));
        for node in nodes.iter() {
            header.push_str(&"│".blue().to_string());
            header.push_str(&format!("{:^width$}", node.green(), width = cell_width));
        }
        header.push_str(&"│".blue().to_string());
        // 构造表头下分界线，左边├，中间┼，右边┤
        let mut header_sep = String::new();
        header_sep.push_str("├");
        header_sep.push_str(&"─".repeat(cell_width));
        for _ in 0..nodes.len() {
            header_sep.push_str("┼");
            header_sep.push_str(&"─".repeat(cell_width));
        }
        header_sep.push_str("┤");
        // 构造数据行，每行第一列为行头（用红色居中），其余为权重，竖线边界也显示为蓝色
        let mut rows = Vec::new();
        for row in nodes.iter() {
            let mut row_str = String::new();
            row_str.push_str(&"│".blue().to_string());
            row_str.push_str(&format!("{:^width$}", row.red(), width = cell_width));
            for col in nodes.iter() {
                row_str.push_str(&"│".blue().to_string());
                let edges = self.adjacency_table().get(*row).unwrap();
                let mut weight_str = " ".to_string();
                for (nbr, weight) in edges.iter() {
                    if nbr == *col {
                        weight_str = weight.to_string();
                        break;
                    }
                }
                if weight_str == " " {
                    row_str.push_str(&format!(
                        "{:^width$}",
                        weight_str.blue(),
                        width = cell_width
                    ));
                } else {
                    row_str.push_str(&format!("{:^width$}", weight_str, width = cell_width));
                }
            }
            row_str.push_str(&"│".blue().to_string());
            rows.push(row_str);
        }
        // 构造行间分隔线，格式与 header_sep 类似
        let mut mid_sep = String::new();
        mid_sep.push_str("├");
        mid_sep.push_str(&"─".repeat(cell_width));
        for _ in 0..nodes.len() {
            mid_sep.push_str("┼");
            mid_sep.push_str(&"─".repeat(cell_width));
        }
        mid_sep.push_str("┤");
        // 构造底边框，左下角└，中间分隔┴，右下角┘
        let mut bottom_border = String::new();
        bottom_border.push_str("└");
        bottom_border.push_str(&"─".repeat(cell_width));
        for _ in 0..nodes.len() {
            bottom_border.push_str("┴");
            bottom_border.push_str(&"─".repeat(cell_width));
        }
        bottom_border.push_str("┘");
        // 打印整个表格，各边框用浅灰色显示
        println!("{}", top_border.blue());
        println!("{}", header);
        println!("{}", header_sep.blue());
        for (i, row) in rows.iter().enumerate() {
            println!("{}", row);
            if i < rows.len() - 1 {
                println!("{}", mid_sep.blue());
            }
        }
        println!("{}", bottom_border.blue());
        // 输出空行
        println!();
    }

    pub fn print_graph_diagram(&self) -> () {
        // 定义画布宽度和高度
        let canvas_width = 80;
        let canvas_height = 40;
        // 初始化画布，用空格填充
        let mut canvas = vec![vec![' '; canvas_width]; canvas_height];

        // 收集所有节点，并排序确保顺序固定
        let mut nodes: Vec<&String> = self.adjacency_table().keys().collect();
        nodes.sort();

        // 设置圆心坐标和半径，保证边缘有一定留白
        let center_x = canvas_width as f64 / 2.0;
        let center_y = canvas_height as f64 / 2.0;
        let radius = (canvas_width.min(canvas_height) as f64 / 2.0) - 5.0;

        // 保存节点在画布中的位置，便于后续连线
        let mut node_positions = std::collections::HashMap::new();
        for (i, node) in nodes.iter().enumerate() {
            // 计算角度，均匀分布
            let angle = 2.0 * std::f64::consts::PI * i as f64 / nodes.len() as f64;
            let x = center_x + radius * angle.cos();
            let y = center_y + radius * angle.sin();
            // 将坐标转换为整数索引
            let ix = x.round() as i32;
            let iy = y.round() as i32;
            node_positions.insert((*node).clone(), (ix, iy));
        }

        // 实现 Bresenham 直线算法绘制连线
        fn bresenham_line(x0: i32, y0: i32, x1: i32, y1: i32) -> Vec<(i32, i32)> {
            // 初始化点集合
            let mut points = Vec::new();
            let mut x = x0;
            let mut y = y0;
            let dx = (x1 - x0).abs();
            let dy = -(y1 - y0).abs();
            let mut err = dx + dy;
            let sx = if x0 < x1 { 1 } else { -1 };
            let sy = if y0 < y1 { 1 } else { -1 };
            loop {
                // 添加当前点
                points.push((x, y));
                if x == x1 && y == y1 {
                    break;
                }
                let e2 = 2 * err;
                if e2 >= dy {
                    err += dy;
                    x += sx;
                }
                if e2 <= dx {
                    err += dx;
                    y += sy;
                }
            }
            points
        }

        // 使用 HashSet 避免重复画同一条无向边
        let mut drawn_edges = std::collections::HashSet::new();
        // 遍历所有节点及相邻边，绘制连线和标注权重
        for (node, pos) in node_positions.iter() {
            // 获取节点的邻接边
            let edges = self.adjacency_table().get(node).unwrap();
            for (nbr, weight) in edges.iter() {
                // 构造排序后的节点对，保证不重复
                let mut edge_nodes = [node.clone(), nbr.clone()];
                edge_nodes.sort();
                if drawn_edges.contains(&(edge_nodes[0].clone(), edge_nodes[1].clone())) {
                    continue;
                }
                drawn_edges.insert((edge_nodes[0].clone(), edge_nodes[1].clone()));
                // 获取相邻节点的位置
                if let Some(&nbr_pos) = node_positions.get(nbr) {
                    // 根据两点坐标计算连线经过的所有点
                    let line_points = bresenham_line(pos.0, pos.1, nbr_pos.0, nbr_pos.1);
                    // 在连线上画点，若该位置为空则填充为点号
                    for (lx, ly) in line_points.iter() {
                        if *ly >= 0
                            && (*ly as usize) < canvas_height
                            && *lx >= 0
                            && (*lx as usize) < canvas_width
                        {
                            if canvas[*ly as usize][*lx as usize] == ' ' {
                                canvas[*ly as usize][*lx as usize] = '.';
                            }
                        }
                    }
                    // 在线的中间位置覆盖权重字符串
                    let mid_index = line_points.len() / 2;
                    let (mx, my) = line_points[mid_index];
                    let weight_str = weight.to_string();
                    for (i, wch) in weight_str.chars().enumerate() {
                        let x_pos = mx + i as i32;
                        if my >= 0
                            && (my as usize) < canvas_height
                            && x_pos >= 0
                            && (x_pos as usize) < canvas_width
                        {
                            canvas[my as usize][x_pos as usize] = wch;
                        }
                    }
                }
            }
        }

        // 绘制所有节点，将节点标签覆盖到各自位置
        for (node, &(x, y)) in node_positions.iter() {
            // 获取节点标签
            let label = node;
            for (i, ch) in label.chars().enumerate() {
                let x_pos = x + i as i32;
                if y >= 0
                    && (y as usize) < canvas_height
                    && x_pos >= 0
                    && (x_pos as usize) < canvas_width
                {
                    canvas[y as usize][x_pos as usize] = ch;
                }
            }
        }

        // 去除上边空白区域
        let mut top = 0;
        while top < canvas_height && canvas[top].iter().all(|&c| c == ' ') {
            top += 1;
        }
        // 去除下边空白区域
        let mut bottom = canvas_height;
        while bottom > top && canvas[bottom - 1].iter().all(|&c| c == ' ') {
            bottom -= 1;
        }
        // 去除左边空白区域
        let mut left = 0;
        'left_loop: for col in 0..canvas_width {
            for row in top..bottom {
                if canvas[row][col] != ' ' {
                    left = col;
                    break 'left_loop;
                }
            }
        }
        // 去除右边空白区域
        let mut right = canvas_width;
        'right_loop: for col in (0..canvas_width).rev() {
            for row in top..bottom {
                if canvas[row][col] != ' ' {
                    right = col + 1;
                    break 'right_loop;
                }
            }
        }
        // 打印去除空白的画布
        for row in top..bottom {
            let mut line = String::new();
            for col in left..right {
                line.push(canvas[row][col]);
            }
            println!("{}", line);
        }
        // 输出一个空行，便于阅读
        println!();
    }
}

pub trait Graph {
    fn new() -> Self;

    fn adjacency_table_mutable(&mut self) -> &mut HashMap<String, Vec<(String, i32)>>;

    fn adjacency_table(&self) -> &HashMap<String, Vec<(String, i32)>>;

    fn add_node(&mut self, node: &str) -> bool {
        // 检查是否已有此节点
        if self.contains(node) {
            return false;
        }
        // 新增空邻接向量
        self.adjacency_table_mutable()
            .insert(node.to_string(), Vec::new());
        true
    }

    fn add_edge(&mut self, edge: (&str, &str, i32)) {
        // 确保节点 a 存在
        if !self.contains(edge.0) {
            self.add_node(edge.0);
        }
        // 确保节点 b 存在
        if !self.contains(edge.1) {
            self.add_node(edge.1);
        }
        // 获取可变邻接表
        let map = self.adjacency_table_mutable();
        // 在 a 中添加 (b, 权重)
        map.get_mut(edge.0)
            .unwrap()
            .push((edge.1.to_string(), edge.2));
        // 在 b 中添加 (a, 权重)
        map.get_mut(edge.1)
            .unwrap()
            .push((edge.0.to_string(), edge.2));
    }

    fn contains(&self, node: &str) -> bool {
        self.adjacency_table().get(node).is_some()
    }

    fn nodes(&self) -> HashSet<&String> {
        self.adjacency_table().keys().collect()
    }

    fn edges(&self) -> Vec<(&String, &String, i32)> {
        let mut edges = Vec::new();
        for (from_node, from_node_neighbours) in self.adjacency_table() {
            for (to_node, weight) in from_node_neighbours {
                edges.push((from_node, to_node, *weight));
            }
        }
        edges
    }
}

#[cfg(test)]
mod test_undirected_graph {
    use super::Graph;
    use super::UndirectedGraph;

    #[test]
    fn test_add_edge() {
        let mut graph = UndirectedGraph::new();
        graph.add_edge(("a", "b", 5));
        // 打印图信息，检查第一次插入状态
        graph.print_graph();
        graph.print_graph_diagram();
        graph.add_edge(("b", "c", 10));
        // 打印图信息，检查第二次插入状态
        graph.print_graph();
        graph.print_graph_diagram();
        graph.add_edge(("c", "a", 7));
        // 打印图信息，检查第三次插入状态
        graph.print_graph();
        graph.print_graph_diagram();
        let expected_edges = [
            (&String::from("a"), &String::from("b"), 5),
            (&String::from("b"), &String::from("a"), 5),
            (&String::from("c"), &String::from("a"), 7),
            (&String::from("a"), &String::from("c"), 7),
            (&String::from("b"), &String::from("c"), 10),
            (&String::from("c"), &String::from("b"), 10),
        ];
        for edge in expected_edges.iter() {
            // 检查边是否存在
            assert_eq!(graph.edges().contains(edge), true);
        }
    }
}
