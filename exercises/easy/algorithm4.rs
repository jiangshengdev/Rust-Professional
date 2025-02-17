/*
    binary_search tree
    This problem requires you to implement a basic interface for a binary tree
*/
/*
    二叉搜索树
    这个问题要求你实现一个基本的二叉树接口
*/

use std::cmp::Ordering;
use std::fmt::Debug;

#[derive(Debug)]
struct TreeNode<T>
where
    T: Ord,
{
    value: T,
    left: Option<Box<TreeNode<T>>>,
    right: Option<Box<TreeNode<T>>>,
}

#[derive(Debug)]
struct BinarySearchTree<T>
where
    T: Ord,
{
    root: Option<Box<TreeNode<T>>>,
}

impl<T> TreeNode<T>
where
    T: Ord,
{
    fn new(value: T) -> Self {
        TreeNode {
            value,
            left: None,
            right: None,
        }
    }
}

impl<T> BinarySearchTree<T>
where
    T: Ord,
{
    fn new() -> Self {
        BinarySearchTree { root: None }
    }

    // Insert a value into the BST
    fn insert(&mut self, value: T) {
        // 若树非空则调用递归插入，否则创建新的根节点
        match self.root {
            Some(ref mut node) => node.insert(value),
            None => self.root = Some(Box::new(TreeNode::new(value))),
        }
    }

    // Search for a value in the BST
    fn search(&self, value: T) -> bool {
        // 从根节点开始递归查找目标值
        fn search_node<T: Ord>(node: &TreeNode<T>, value: &T) -> bool {
            match node.value.cmp(value) {
                Ordering::Equal => {
                    // 找到目标值，返回 true
                    true
                }
                Ordering::Greater =>
                // 当前节点大于目标值，检查左子树
                {
                    node.left.as_ref().is_some_and(|n| {
                        // 左子节点存在，递归查找目标值
                        search_node(n, value)
                    })
                }
                Ordering::Less =>
                // 当前节点小于目标值，检查右子树
                {
                    node.right.as_ref().is_some_and(|n| {
                        // 右子节点存在，递归查找目标值
                        search_node(n, value)
                    })
                }
            }
        }
        match &self.root {
            Some(node) => search_node(node, &value),
            None => false, // 空树直接返回 false
        }
    }
}

impl<T> TreeNode<T>
where
    T: Ord,
{
    // Insert a node into the tree
    fn insert(&mut self, value: T) {
        match self.value.cmp(&value) {
            Ordering::Greater => {
                // 当前节点大于待插入值，则尝试插入到左子树中
                if let Some(ref mut left) = self.left {
                    left.insert(value);
                } else {
                    // 左子树为空，直接创建新节点
                    self.left = Some(Box::new(TreeNode::new(value)));
                }
            }
            Ordering::Less => {
                // 当前节点小于待插入值，则尝试插入到右子树中
                if let Some(ref mut right) = self.right {
                    right.insert(value);
                } else {
                    // 右子树为空，直接创建新节点
                    self.right = Some(Box::new(TreeNode::new(value)));
                }
            }
            Ordering::Equal => {
                // 值相同，不插入重复节点
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert_and_search() {
        let mut bst = BinarySearchTree::new();

        assert_eq!(bst.search(1), false);

        bst.insert(5);
        bst.insert(3);
        bst.insert(7);
        bst.insert(2);
        bst.insert(4);

        assert_eq!(bst.search(5), true);
        assert_eq!(bst.search(3), true);
        assert_eq!(bst.search(7), true);
        assert_eq!(bst.search(2), true);
        assert_eq!(bst.search(4), true);

        assert_eq!(bst.search(1), false);
        assert_eq!(bst.search(6), false);
    }

    #[test]
    fn test_insert_duplicate() {
        let mut bst = BinarySearchTree::new();

        bst.insert(1);
        bst.insert(1);

        assert_eq!(bst.search(1), true);

        match bst.root {
            Some(ref node) => {
                assert!(node.left.is_none());
                assert!(node.right.is_none());
            }
            None => panic!("Root should not be None after insertion"),
        }
    }
}
