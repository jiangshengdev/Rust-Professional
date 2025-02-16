/*
    Rotate Matrix 90 Degrees
    Given a 2D matrix, rotate it 90 degrees in place.
    You need to perform the rotation without using any additional matrix storage.

    You need to implement the function `rotate_matrix_90_degrees(matrix: &mut Vec<Vec<i32>>)`.
    The function should rotate the input matrix in place.

    Hint: Consider rotating the matrix layer by layer, starting from the outermost layer and working your way inward.
*/
/*
    旋转矩阵90度
    给定一个二维矩阵，原地旋转90度。
    必须在不使用额外矩阵存储的情况下完成旋转。

    需要实现函数 `rotate_matrix_90_degrees(matrix: &mut Vec<Vec<i32>>)`。
    该函数应原地旋转输入矩阵。

    提示：考虑从最外层开始逐层旋转，向内逐层处理。
*/

use std::fmt::{self, Display, Formatter};

pub fn rotate_matrix_90_degrees(matrix: &mut Vec<Vec<i32>>) {
    // 如果矩阵为空则直接返回
    if matrix.is_empty() {
        return;
    }

    // 获取原始矩阵行数和列数
    let original_rows = matrix.len();
    let original_cols = matrix[0].len();

    // 计算扩展后正方形的边长
    let d = original_rows.max(original_cols);

    // 补全每行到 d 个元素
    for row in matrix.iter_mut() {
        // 补充不足的部分，每一行补零
        while row.len() < d {
            row.push(0);
        }
    }

    // 如果行数不足 d，则添加新行，并且每一行填充 d 个零
    while matrix.len() < d {
        // 构造一个包含 d 个零的新行
        let new_row = vec![0; d];
        matrix.push(new_row);
    }

    // 对正方形矩阵进行转置，交换对角线两侧的元素
    for i in 0..d {
        for j in (i + 1)..d {
            // 交换 matrix[i][j] 与 matrix[j][i]
            let temp = matrix[i][j];
            matrix[i][j] = matrix[j][i];
            matrix[j][i] = temp;
        }
    }

    // 反转每一行实现顺时针旋转90度
    for row in matrix.iter_mut() {
        row.reverse();
    }

    // 去掉补充的行，保留前 original_cols 行
    matrix.truncate(original_cols);

    // 每一行保留前 original_rows 个元素
    for row in matrix.iter_mut() {
        row.truncate(original_rows);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rotate_matrix_1() {
        let mut matrix = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
        rotate_matrix_90_degrees(&mut matrix);
        println!("Rotated matrix: {:?}", matrix);
        assert_eq!(matrix, vec![vec![7, 4, 1], vec![8, 5, 2], vec![9, 6, 3],]);
    }

    #[test]
    fn test_rotate_matrix_2() {
        let mut matrix = vec![vec![1, 2], vec![3, 4]];
        rotate_matrix_90_degrees(&mut matrix);
        println!("Rotated matrix: {:?}", matrix);
        assert_eq!(matrix, vec![vec![3, 1], vec![4, 2],]);
    }

    #[test]
    fn test_rotate_matrix_3() {
        let mut matrix = vec![vec![1]];
        rotate_matrix_90_degrees(&mut matrix);
        println!("Rotated matrix: {:?}", matrix);
        assert_eq!(matrix, vec![vec![1],]);
    }

    #[test]
    fn test_rotate_matrix_4() {
        let mut matrix = vec![vec![1, 2], vec![3, 4], vec![5, 6]];
        rotate_matrix_90_degrees(&mut matrix);
        println!("Rotated matrix: {:?}", matrix);
        assert_eq!(matrix, vec![vec![5, 3, 1], vec![6, 4, 2],]);
    }
}
