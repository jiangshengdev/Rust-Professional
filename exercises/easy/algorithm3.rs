/*
    sort
    This problem requires you to implement a sorting algorithm
    you can use bubble sorting, insertion sorting, heap sorting, etc.
*/

fn sort<T: Ord>(array: &mut [T]) {
    quicksort(array);
}

// 使用快速排序递归数组
fn quicksort<T: Ord>(arr: &mut [T]) {
    if arr.len() <= 1 {
        return;
    }
    // 选最后一个元素为 pivot, 分区并获取 pivot 最终位置
    let pivot_index = partition(arr);
    // 分割数组: 左侧为小于等于 pivot 的部分, 右侧为大于 pivot 的部分 (右侧第一个为 pivot)
    let (left, right) = arr.split_at_mut(pivot_index);
    quicksort(left);
    // 右侧若多于一个元素, 排序除 pivot 外的部分
    if right.len() > 1 {
        quicksort(&mut right[1..]);
    }
}

// 使用最后一个元素作为 pivot, 并调整数组, 返回 pivot 的最终位置
fn partition<T: Ord>(arr: &mut [T]) -> usize {
    let pivot_index = arr.len() - 1;
    let pivot = &arr[pivot_index];
    let mut i = 0;
    // 遍历除 pivot 外的所有元素, 小于等于 pivot 的交换到左侧
    for j in 0..pivot_index {
        if &arr[j] <= pivot {
            arr.swap(i, j);
            i += 1;
        }
    }
    // 将 pivot 放到正确位置
    arr.swap(i, pivot_index);
    i
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sort_1() {
        let mut vec = vec![37, 73, 57, 75, 91, 19, 46, 64];
        sort(&mut vec);
        assert_eq!(vec, vec![19, 37, 46, 57, 64, 73, 75, 91]);
    }
    #[test]
    fn test_sort_2() {
        let mut vec = vec![1];
        sort(&mut vec);
        assert_eq!(vec, vec![1]);
    }
    #[test]
    fn test_sort_3() {
        let mut vec = vec![99, 88, 77, 66, 55, 44, 33, 22, 11];
        sort(&mut vec);
        assert_eq!(vec, vec![11, 22, 33, 44, 55, 66, 77, 88, 99]);
    }
}
