//! # 119. 杨辉三角 II
//!
//! 难度 简单
//!
//! 给定一个非负索引 k，其中 k ≤ 33，返回杨辉三角的第 k 行。
//!
//!
//!
//! 在杨辉三角中，每个数是它左上方和右上方的数的和。
//!
//! ## 示例:
//!
//! ```text
//! 输入: 3
//! 输出: [1,3,3,1]
//! ```
//! ## 进阶：
//!
//! 你可以优化你的算法到 *O(k)* 空间复杂度吗？
//!
//! See [leetcode](https://leetcode-cn.com/problems/pascals-triangle-ii/)

pub struct Solution;
impl Solution {
    pub fn get_row(row_index: i32) -> Vec<i32> {
        assert!(row_index >= 0);

        gen_pascal_triangle()
            .skip(row_index as usize)
            .next()
            .unwrap()
    }
}

fn gen_pascal_triangle() -> impl Iterator<Item = Vec<i32>> {
    std::iter::successors(Some(vec![1]), |row| {
        vec![1].into_iter()
            .chain(row.windows(2).map(|v| v.iter().sum()))
            .chain(vec![1].into_iter())
            .collect::<Vec<i32>>()
            .into()
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let cases = vec![
            (0, vec![1]),
            (1, vec![1,1]),
            (2, vec![1,2,1]),
            (3, vec![1,3,3,1]),
            (4, vec![1,4,6,4,1]),
        ];

        for (num_rows, expect) in cases {
            assert_eq!(expect, Solution::get_row(num_rows));
        }
    }
}
