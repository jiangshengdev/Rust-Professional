/*
    queue
    This question requires you to use queues to implement the functionality of the stac
*/
/*
    队列
    本题要求你使用队列来实现栈的功能
*/

#[derive(Debug)]
pub struct Queue<T> {
    elements: Vec<T>,
}

impl<T> Queue<T> {
    pub fn new() -> Queue<T> {
        Queue {
            elements: Vec::new(),
        }
    }

    pub fn enqueue(&mut self, value: T) {
        self.elements.push(value)
    }

    pub fn dequeue(&mut self) -> Result<T, &str> {
        if !self.elements.is_empty() {
            Ok(self.elements.remove(0usize))
        } else {
            Err("Queue is empty")
        }
    }

    pub fn peek(&self) -> Result<&T, &str> {
        match self.elements.first() {
            Some(value) => Ok(value),
            None => Err("Queue is empty"),
        }
    }

    pub fn size(&self) -> usize {
        self.elements.len()
    }

    pub fn is_empty(&self) -> bool {
        self.elements.is_empty()
    }
}

impl<T> Default for Queue<T> {
    fn default() -> Queue<T> {
        Queue {
            elements: Vec::new(),
        }
    }
}

pub struct MyStack<T> {
    q1: Queue<T>,
    q2: Queue<T>,
}

impl<T> Default for MyStack<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> MyStack<T> {
    pub fn new() -> Self {
        Self {
            q1: Queue::<T>::new(),
            q2: Queue::<T>::new(),
        }
    }

    pub fn push(&mut self, elem: T) {
        // 插入新元素到辅助队列 q2 中
        self.q2.enqueue(elem);
        // 将主队列 q1 中所有元素依次转移到 q2 中
        while !self.q1.is_empty() {
            // 从 q1 中移除第一个元素
            match self.q1.dequeue() {
                Ok(val) => {
                    // 将移除的元素添加到 q2 中
                    self.q2.enqueue(val);
                }
                Err(_) => {
                    // 队列非空时不应失败，若出现异常直接退出循环
                    break;
                }
            }
        }
        // 交换 q1 与 q2，确保栈顶元素始终在 q1 的头部
        std::mem::swap(&mut self.q1, &mut self.q2);
    }

    pub fn pop(&mut self) -> Result<T, &str> {
        // 判断栈是否为空
        if self.q1.is_empty() {
            // 栈空时返回错误
            Err("Stack is empty")
        } else {
            // 栈非空时，从 q1 中移除并返回元素
            self.q1.dequeue()
        }
    }

    pub fn is_empty(&self) -> bool {
        // 返回 q1 是否为空，空表示栈为空
        self.q1.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_queue() {
        let mut s = MyStack::<i32>::new();
        assert_eq!(s.pop(), Err("Stack is empty"));
        s.push(1);
        s.push(2);
        s.push(3);
        assert_eq!(s.pop(), Ok(3));
        assert_eq!(s.pop(), Ok(2));
        s.push(4);
        s.push(5);
        assert!(!s.is_empty());
        assert_eq!(s.pop(), Ok(5));
        assert_eq!(s.pop(), Ok(4));
        assert_eq!(s.pop(), Ok(1));
        assert_eq!(s.pop(), Err("Stack is empty"));
        assert!(s.is_empty());
    }
}
