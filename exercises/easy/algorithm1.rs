/*
    single linked list merge
    This problem requires you to merge two ordered singly linked lists into one ordered singly linked list
*/
/*
    单链表合并
    这个问题要求你将两个有序的单链表合并为一个有序的单链表
*/

use std::fmt::{self, Display, Formatter};
use std::ptr::NonNull;

#[derive(Debug)]
struct Node<T> {
    val: T,
    next: Option<NonNull<Node<T>>>,
}

impl<T> Node<T> {
    fn new(t: T) -> Node<T> {
        Node { val: t, next: None }
    }
}

#[derive(Debug)]
struct LinkedList<T> {
    length: u32,
    start: Option<NonNull<Node<T>>>,
    end: Option<NonNull<Node<T>>>,
}

impl<T> Default for LinkedList<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        Self {
            length: 0,
            start: None,
            end: None,
        }
    }

    pub fn add(&mut self, obj: T) {
        // 新建一个节点并初始化，节点的 next 指针默认为 None
        let mut node = Box::new(Node::new(obj));
        node.next = None;
        // 将节点转换为裸指针并包装到 Some 中
        let node_ptr = Some(unsafe { NonNull::new_unchecked(Box::into_raw(node)) });
        match self.end {
            // 如果链表为空，则新节点既为头节点也为尾节点
            None => self.start = node_ptr,
            // 否则将当前尾节点的 next 更新为指向新节点
            Some(end_ptr) => unsafe { (*end_ptr.as_ptr()).next = node_ptr },
        }
        // 更新链表尾指针为新节点，并增加链表长度
        self.end = node_ptr;
        self.length += 1;
    }

    pub fn get(&mut self, index: i32) -> Option<&T> {
        self.get_ith_node(self.start, index)
    }

    fn get_ith_node(&mut self, node: Option<NonNull<Node<T>>>, index: i32) -> Option<&T> {
        // 如果当前节点为空，则说明索引超出范围，返回 None
        match node {
            None => None,
            Some(current_ptr) => match index {
                // 当 index 为 0 时，返回当前节点的值
                0 => Some(unsafe { &(*current_ptr.as_ptr()).val }),
                // 否则递归查找下一个节点，索引递减
                _ => self.get_ith_node(unsafe { (*current_ptr.as_ptr()).next }, index - 1),
            },
        }
    }

    pub fn merge(list_a: LinkedList<T>, list_b: LinkedList<T>) -> Self
    where
        T: Clone + PartialOrd,
    {
        // 创建存放合并结果的新链表
        let mut new_list = LinkedList::<T>::new();
        let mut a_ptr = list_a.start;
        let mut b_ptr = list_b.start;

        unsafe {
            // 逐个比较两个链表中的节点，将较小的元素加入 new_list 中
            while let (Some(a), Some(b)) = (a_ptr, b_ptr) {
                if (*a.as_ptr()).val <= (*b.as_ptr()).val {
                    new_list.add((*a.as_ptr()).val.clone());
                    a_ptr = (*a.as_ptr()).next;
                } else {
                    new_list.add((*b.as_ptr()).val.clone());
                    b_ptr = (*b.as_ptr()).next;
                }
            }
            // 如果 list_a 中还有剩余节点，则将它们全部添加到 new_list 中
            while let Some(a) = a_ptr {
                new_list.add((*a.as_ptr()).val.clone());
                a_ptr = (*a.as_ptr()).next;
            }
            // 如果 list_b 中还有剩余节点，则将它们全部添加到 new_list 中
            while let Some(b) = b_ptr {
                new_list.add((*b.as_ptr()).val.clone());
                b_ptr = (*b.as_ptr()).next;
            }
        }
        // 返回合并后的链表
        new_list
    }
}

impl<T> Display for LinkedList<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self.start {
            Some(node) => write!(f, "{}", unsafe { node.as_ref() }),
            None => Ok(()),
        }
    }
}

impl<T> Display for Node<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self.next {
            Some(node) => write!(f, "{}, {}", self.val, unsafe { node.as_ref() }),
            None => write!(f, "{}", self.val),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::LinkedList;

    #[test]
    fn create_numeric_list() {
        let mut list = LinkedList::<i32>::new();
        list.add(1);
        list.add(2);
        list.add(3);
        println!("Linked List is {}", list);
        assert_eq!(3, list.length);
    }

    #[test]
    fn create_string_list() {
        let mut list_str = LinkedList::<String>::new();
        list_str.add("A".to_string());
        list_str.add("B".to_string());
        list_str.add("C".to_string());
        println!("Linked List is {}", list_str);
        assert_eq!(3, list_str.length);
    }

    #[test]
    fn test_merge_linked_list_1() {
        let mut list_a = LinkedList::<i32>::new();
        let mut list_b = LinkedList::<i32>::new();
        let vec_a = vec![1, 3, 5, 7];
        let vec_b = vec![2, 4, 6, 8];
        let target_vec = vec![1, 2, 3, 4, 5, 6, 7, 8];

        for i in 0..vec_a.len() {
            list_a.add(vec_a[i]);
        }
        for i in 0..vec_b.len() {
            list_b.add(vec_b[i]);
        }
        println!("list a {} list b {}", list_a, list_b);
        let mut list_c = LinkedList::<i32>::merge(list_a, list_b);
        println!("merged List is {}", list_c);
        for i in 0..target_vec.len() {
            assert_eq!(target_vec[i], *list_c.get(i as i32).unwrap());
        }
    }

    #[test]
    fn test_merge_linked_list_2() {
        let mut list_a = LinkedList::<i32>::new();
        let mut list_b = LinkedList::<i32>::new();
        let vec_a = vec![11, 33, 44, 88, 89, 90, 100];
        let vec_b = vec![1, 22, 30, 45];
        let target_vec = vec![1, 11, 22, 30, 33, 44, 45, 88, 89, 90, 100];

        for i in 0..vec_a.len() {
            list_a.add(vec_a[i]);
        }
        for i in 0..vec_b.len() {
            list_b.add(vec_b[i]);
        }
        println!("list a {} list b {}", list_a, list_b);
        let mut list_c = LinkedList::<i32>::merge(list_a, list_b);
        println!("merged List is {}", list_c);
        for i in 0..target_vec.len() {
            assert_eq!(target_vec[i], *list_c.get(i as i32).unwrap());
        }
    }
}
