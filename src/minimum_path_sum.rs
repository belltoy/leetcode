//! # 64. 最小路径和
//!
//! 难度 中等
//!
//! 给定一个包含非负整数的 m x n 网格 grid ，请找出一条从左上角到右下角的路径，使得路径上的数字总和为最小。
//!
//! 说明：每次只能向下或者向右移动一步。
//!
//!
//! ## 示例 1：
//!
//!
//! ```text
//! 输入：grid = [[1,3,1],[1,5,1],[4,2,1]]
//! 输出：7
//! 解释：因为路径 1→3→1→1→1 的总和最小。
//! ```
//!
//! ## 示例 2：
//!
//! ```text
//! 输入：grid = [[1,2,3],[4,5,6]]
//! 输出：12
//! ```
//!
//! ## 提示：
//!
//! * `m == grid.length`
//! * `n == grid[i].length`
//! * `1 <= m, n <= 200`
//! * `0 <= grid[i][j] <= 100`
//!
//! See [leetcode](https://leetcode-cn.com/problems/minimum-path-sum/)

pub struct Solution;

impl Solution {
    pub fn min_path_sum(grid: Vec<Vec<i32>>) -> i32 {
        assert!(grid.len() >= 1 && grid.len() <= 200);
        assert!(grid[0].len() >= 1 && grid[0].len() <= 200);

        grid.iter().fold(None::<Vec<i32>>, |last_row, row| {
            row.iter().enumerate().scan(None, |left, (i, &v)| {
                assert!(v >= 0 && v <= 100);   // assume never overflow
                *left = match (&left, &last_row) {
                    (None, None) => v,                         // first cell
                    (Some(left), None) => left + v,            // first row
                    (None, Some(last_row)) => last_row[i] + v, // first column
                    (Some(left), Some(last_row)) => std::cmp::min(*left, last_row[i]) + v,
                }.into();
                *left
            }).collect::<Vec<_>>().into()
        }).unwrap().pop().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let cases = vec![
            (7, vec![vec![1,3,1],vec![1,5,1],vec![4,2,1]]),
            (12, vec![vec![1,2,3],vec![4,5,6]]),
            (14, vec![vec![1,3,4,8], vec![3,2,2,4], vec![5,7,1,9], vec![2,3,2,3]]),
        ];

        for (expect, input) in cases {
            assert_eq!(expect, Solution::min_path_sum(input));
        }
    }
}
