/*
    Rotate Matrix 90 Degrees
    Given a 2D matrix, rotate it 90 degrees in place. 
    You need to perform the rotation without using any additional matrix storage.

    You need to implement the function `rotate_matrix_90_degrees(matrix: &mut Vec<Vec<i32>>)`.
    The function should rotate the input matrix in place.

    Hint: Consider rotating the matrix layer by layer, starting from the outermost layer and working your way inward.
*/

use std::fmt::{self, Display, Formatter};

pub fn rotate_matrix_90_degrees(matrix: &mut Vec<Vec<i32>>) {
    let rows = matrix.len();
    if rows == 0 { return; }
    let cols = matrix[0].len();
    
    // 非正方形矩阵需要特殊处理
    if rows != cols {
        // 旋转后矩阵尺寸变为 cols x rows
        // 需要重新构建矩阵
        let mut rotated = Vec::with_capacity(cols);
        for j in 0..cols {
            let mut new_row = Vec::with_capacity(rows);
            for i in (0..rows).rev() {
                new_row.push(matrix[i][j]);
            }
            rotated.push(new_row);
        }
        *matrix = rotated;
        return;
    }

    // 正方形矩阵原地旋转
    let n = rows;
    for layer in 0..n/2 {
        let first = layer;
        let last = n - 1 - layer;
        for i in first..last {
            let offset = i - first;
            // 保存上边
            let top = matrix[first][i];
            // 左到上
            matrix[first][i] = matrix[last-offset][first];
            // 下到左
            matrix[last-offset][first] = matrix[last][last-offset];
            // 右到下
            matrix[last][last-offset] = matrix[i][last];
            // 上到右
            matrix[i][last] = top;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rotate_matrix_1() {
        let mut matrix = vec![
            vec![1, 2, 3],
            vec![4, 5, 6],
            vec![7, 8, 9],
        ];
        rotate_matrix_90_degrees(&mut matrix);
        assert_eq!(
            matrix,
            vec![
                vec![7, 4, 1],
                vec![8, 5, 2],
                vec![9, 6, 3],
            ]
        );
    }

    #[test]
    fn test_rotate_matrix_2() {
        let mut matrix = vec![
            vec![1, 2],
            vec![3, 4],
        ];
        rotate_matrix_90_degrees(&mut matrix);
        assert_eq!(
            matrix,
            vec![
                vec![3, 1],
                vec![4, 2],
            ]
        );
    }

    #[test]
    fn test_rotate_matrix_3() {
        let mut matrix = vec![
            vec![1],
        ];
        rotate_matrix_90_degrees(&mut matrix);
        assert_eq!(
            matrix,
            vec![
                vec![1],
            ]
        );
    }

    #[test]
    fn test_rotate_matrix_4() {
        let mut matrix = vec![
            vec![1, 2],
            vec![3, 4],
            vec![5, 6],
        ];
        rotate_matrix_90_degrees(&mut matrix);
        assert_eq!(
            matrix,
            vec![
                vec![5, 3, 1],
                vec![6, 4, 2],
            ]
        );
    }

    #[test]
    fn test_rotate_rectangular_matrix() {
        let mut matrix = vec![
            vec![1, 2, 3],
            vec![4, 5, 6],
        ];
        rotate_matrix_90_degrees(&mut matrix);
        assert_eq!(
            matrix,
            vec![
                vec![4, 1],
                vec![5, 2],
                vec![6, 3],
            ]
        );
    }
}