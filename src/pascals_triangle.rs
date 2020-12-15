//! # 118. 杨辉三角
//!
//! 难度 简单
//!
//! 给定一个非负整数 numRows，生成杨辉三角的前 numRows 行。
//!
//! 在杨辉三角中，每个数是它左上方和右上方的数的和。
//!
//! ## 示例:
//!
//! ```text
//! 输入: 5
//! 输出:
//! [
//!      [1],
//!     [1,1],
//!    [1,2,1],
//!   [1,3,3,1],
//!  [1,4,6,4,1]
//! ]
//! ```
//! See [leetcode](https://leetcode-cn.com/problems/pascals-triangle/)

pub struct Solution;
impl Solution {
    pub fn generate(num_rows: i32) -> Vec<Vec<i32>> {
        assert!(num_rows >= 0);

        gen_pascal_triangle()
            .take(num_rows as usize)
            .collect()
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
            (0, vec![]),
            (1, vec![vec![1]]),
            (2, vec![
                    vec![1],
                    vec![1,1],
            ]),
            (5, vec![
                    vec![1],
                    vec![1,1],
                    vec![1,2,1],
                    vec![1,3,3,1],
                    vec![1,4,6,4,1]]),
        ];

        for (num_rows, expect) in cases {
            assert_eq!(expect, Solution::generate(num_rows));
        }
    }

    #[test]
    fn test_gen_pascal_triangle() {
        let mut gen = gen_pascal_triangle();
        assert_eq!(vec![1], gen.next().unwrap());
        assert_eq!(vec![1,1], gen.next().unwrap());
        assert_eq!(vec![1,2,1], gen.next().unwrap());
        assert_eq!(vec![1,3,3,1], gen.next().unwrap());
        assert_eq!(vec![1,4,6,4,1], gen.next().unwrap());
    }
}
