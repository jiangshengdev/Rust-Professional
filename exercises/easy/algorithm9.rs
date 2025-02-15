/*
    heap
    This question requires you to implement a binary heap function
*/

use std::cmp::Ord;
use std::default::Default;

pub struct Heap<T>
where
    T: Default,
{
    count: usize,
    items: Vec<T>,
    comparator: fn(&T, &T) -> bool,
}

impl<T> Heap<T>
where
    T: Default,
{
    pub fn new(comparator: fn(&T, &T) -> bool) -> Self {
        Self {
            count: 0,
            items: vec![T::default()],
            comparator,
        }
    }

    pub fn len(&self) -> usize {
        self.count
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn add(&mut self, value: T) {
        // 插入新值到末尾
        self.items.push(value);
        // 更新堆中元素个数
        self.count += 1;
        // 从新加入的节点开始向上调整
        let mut idx = self.count;
        // 开始上浮过程
        while idx > 1 {
            // 先计算父节点索引
            let parent_idx = self.parent_idx(idx);
            // 决定是否需要交换
            let should_swap = (self.comparator)(&self.items[idx], &self.items[parent_idx]);
            if should_swap {
                // 执行交换操作
                self.items.swap(idx, parent_idx);
                // 更新当前节点索引
                idx = parent_idx;
            } else {
                // 完成上浮
                break;
            }
        }
    }

    fn parent_idx(&self, idx: usize) -> usize {
        idx / 2
    }

    fn children_present(&self, idx: usize) -> bool {
        self.left_child_idx(idx) <= self.count
    }

    fn left_child_idx(&self, idx: usize) -> usize {
        idx * 2
    }

    fn right_child_idx(&self, idx: usize) -> usize {
        self.left_child_idx(idx) + 1
    }

    fn smallest_child_idx(&self, idx: usize) -> usize {
        // 获取左孩子索引
        let left = self.left_child_idx(idx);
        let right = self.right_child_idx(idx);
        // 如果右孩子存在，则两者之间选择比较优先的一个
        if right <= self.count {
            // 比较左右孩子，选择符合比较器规则的孩子
            if (self.comparator)(&self.items[left], &self.items[right]) {
                // 左孩子较优
                left
            } else {
                // 右孩子较优
                right
            }
        } else {
            // 只有左孩子存在，直接返回左孩子索引
            left
        }
    }
}

impl<T> Heap<T>
where
    T: Default + Ord,
{
    /// Create a new MinHeap
    pub fn new_min() -> Self {
        Self::new(|a, b| a < b)
    }

    /// Create a new MaxHeap
    pub fn new_max() -> Self {
        Self::new(|a, b| a > b)
    }
}

impl<T> Iterator for Heap<T>
where
    T: Default,
{
    type Item = T;

    fn next(&mut self) -> Option<T> {
        // 堆为空时返回 None
        if self.count == 0 {
            return None;
        }
        // 交换堆顶和末尾元素
        self.items.swap(1, self.count);
        // 弹出堆顶元素
        let res = self.items.pop();
        // 更新堆中元素个数
        self.count -= 1;
        // 从堆顶开始向下调整
        let mut idx = 1;
        // 当左孩子存在时进行下沉过程
        while self.left_child_idx(idx) <= self.count {
            // 选择左右孩子中比较优先的一个
            let child_idx = self.smallest_child_idx(idx);
            // 如果当前节点和孩子节点交换后能满足比较器规则，则交换
            if (self.comparator)(&self.items[child_idx], &self.items[idx]) {
                // 交换当前节点与选中孩子
                self.items.swap(idx, child_idx);
                // 更新当前节点索引为孩子索引
                idx = child_idx;
            } else {
                // 调整完成，退出循环
                break;
            }
        }
        // 返回原堆顶元素
        res
    }
}

pub struct MinHeap;

impl MinHeap {
    #[allow(clippy::new_ret_no_self)]
    pub fn new<T>() -> Heap<T>
    where
        T: Default + Ord,
    {
        Heap::new(|a, b| a < b)
    }
}

pub struct MaxHeap;

impl MaxHeap {
    #[allow(clippy::new_ret_no_self)]
    pub fn new<T>() -> Heap<T>
    where
        T: Default + Ord,
    {
        Heap::new(|a, b| a > b)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_empty_heap() {
        let mut heap = MaxHeap::new::<i32>();
        assert_eq!(heap.next(), None);
    }

    #[test]
    fn test_min_heap() {
        let mut heap = MinHeap::new();
        heap.add(4);
        heap.add(2);
        heap.add(9);
        heap.add(11);
        assert_eq!(heap.len(), 4);
        assert_eq!(heap.next(), Some(2));
        assert_eq!(heap.next(), Some(4));
        assert_eq!(heap.next(), Some(9));
        heap.add(1);
        assert_eq!(heap.next(), Some(1));
    }

    #[test]
    fn test_max_heap() {
        let mut heap = MaxHeap::new();
        heap.add(4);
        heap.add(2);
        heap.add(9);
        heap.add(11);
        assert_eq!(heap.len(), 4);
        assert_eq!(heap.next(), Some(11));
        assert_eq!(heap.next(), Some(9));
        assert_eq!(heap.next(), Some(4));
        heap.add(1);
        assert_eq!(heap.next(), Some(2));
    }
}
