/*
    stack
    This question requires you to use a stack to achieve a bracket match
*/
/*
    stack
    这个问题要求你使用栈来实现括号匹配
*/

#[derive(Debug)]
struct Stack<T> {
    size: usize,
    data: Vec<T>,
}

impl<T> Stack<T> {
    fn new() -> Self {
        Self {
            size: 0,
            data: Vec::new(),
        }
    }

    fn is_empty(&self) -> bool {
        0 == self.size
    }

    fn len(&self) -> usize {
        self.size
    }

    fn clear(&mut self) {
        self.size = 0;
        self.data.clear();
    }

    fn push(&mut self, val: T) {
        self.data.push(val);
        self.size += 1;
    }

    fn pop(&mut self) -> Option<T> {
        // 判断栈是否为空；若为空，则直接返回 None
        if self.is_empty() {
            None
        } else {
            // 弹出栈顶元素，并更新 size
            self.size -= 1;
            self.data.pop()
        }
    }

    fn peek(&self) -> Option<&T> {
        if 0 == self.size {
            return None;
        }
        self.data.get(self.size - 1)
    }

    fn peek_mut(&mut self) -> Option<&mut T> {
        if 0 == self.size {
            return None;
        }
        self.data.get_mut(self.size - 1)
    }

    fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }

    fn iter(&self) -> Iter<T> {
        let mut iterator = Iter { stack: Vec::new() };
        for item in self.data.iter() {
            iterator.stack.push(item);
        }
        iterator
    }

    fn iter_mut(&mut self) -> IterMut<T> {
        let mut iterator = IterMut { stack: Vec::new() };
        for item in self.data.iter_mut() {
            iterator.stack.push(item);
        }
        iterator
    }
}

struct IntoIter<T>(Stack<T>);

impl<T: Clone> Iterator for IntoIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        if !self.0.is_empty() {
            self.0.size -= 1;
            self.0.data.pop()
        } else {
            None
        }
    }
}

struct Iter<'a, T: 'a> {
    stack: Vec<&'a T>,
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        self.stack.pop()
    }
}

struct IterMut<'a, T: 'a> {
    stack: Vec<&'a mut T>,
}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;
    fn next(&mut self) -> Option<Self::Item> {
        self.stack.pop()
    }
}

fn bracket_match(bracket: &str) -> bool {
    // 初始化一个空栈存储括号字符
    let mut stack = Stack::new();
    // 遍历输入字符串中的每个字符
    for ch in bracket.chars() {
        // 根据当前字符进行判断
        match ch {
            // 当遇到左括号时，将其压入栈中
            '(' | '{' | '[' => {
                // 把左括号入栈
                stack.push(ch);
            }
            // 当遇到右括号时
            ')' | '}' | ']' => {
                // 尝试从栈中弹出一个左括号
                if let Some(open) = stack.pop() {
                    // 检查弹出的左括号与当前右括号是否匹配
                    match (open, ch) {
                        // 若匹配则继续处理下一个字符
                        ('(', ')') | ('{', '}') | ('[', ']') => {
                            // ...匹配成功，继续...
                        }
                        // 如果不匹配，说明括号不匹配，返回 false
                        _ => return false,
                    }
                } else {
                    // 如果栈为空，则没有对应的左括号，返回 false
                    return false;
                }
            }
            // 其他字符直接忽略
            _ => {
                // ...无需处理其他字符...
            }
        }
    }
    // 遍历完所有字符后，若栈为空，则所有括号匹配正确
    stack.is_empty()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bracket_matching_1() {
        let s = "(2+3){func}[abc]";
        assert_eq!(bracket_match(s), true);
    }

    #[test]
    fn bracket_matching_2() {
        let s = "(2+3)*(3-1";
        assert_eq!(bracket_match(s), false);
    }

    #[test]
    fn bracket_matching_3() {
        let s = "{{([])}}";
        assert_eq!(bracket_match(s), true);
    }

    #[test]
    fn bracket_matching_4() {
        let s = "{{(}[)]}";
        assert_eq!(bracket_match(s), false);
    }

    #[test]
    fn bracket_matching_5() {
        let s = "[[[]]]]]]]]]";
        assert_eq!(bracket_match(s), false);
    }

    #[test]
    fn bracket_matching_6() {
        let s = "";
        assert_eq!(bracket_match(s), true);
    }
}
