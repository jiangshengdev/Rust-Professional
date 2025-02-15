/*
    graph
    This problem requires you to implement a basic graph functio
*/

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
        graph.add_edge(("b", "c", 10));
        graph.add_edge(("c", "a", 7));
        let expected_edges = [
            (&String::from("a"), &String::from("b"), 5),
            (&String::from("b"), &String::from("a"), 5),
            (&String::from("c"), &String::from("a"), 7),
            (&String::from("a"), &String::from("c"), 7),
            (&String::from("b"), &String::from("c"), 10),
            (&String::from("c"), &String::from("b"), 10),
        ];
        for edge in expected_edges.iter() {
            assert_eq!(graph.edges().contains(edge), true);
        }
    }
}
