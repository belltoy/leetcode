//! # 509. 斐波那契数
//!
//! 难度 简单
//!
//! 斐波那契数，通常用 F(n) 表示，形成的序列称为斐波那契数列。该数列由 0 和 1 开始，后面的每一项数字都是前面两项数字的和。也就是：
//!
//! ```text
//! F(0) = 0,   F(1) = 1
//! F(N) = F(N - 1) + F(N - 2), 其中 N > 1.
//! ```
//!
//! 给定 N，计算 F(N)。
//!
//!
//! ## 示例 1：
//!
//! ```text
//! 输入：2
//! 输出：1
//! 解释：F(2) = F(1) + F(0) = 1 + 0 = 1.
//! ```
//!
//! ## 示例 2：
//!
//! ```text
//! 输入：3
//! 输出：2
//! 解释：F(3) = F(2) + F(1) = 1 + 1 = 2.
//! ```
//!
//! ## 示例 3：
//!
//! ```text
//! 输入：4
//! 输出：3
//! 解释：F(4) = F(3) + F(2) = 2 + 1 = 3.
//! ```
//!
//! ## 提示：
//!
//! * `0 ≤ N ≤ 30`
//!
//! See [leetcode](https://leetcode-cn.com/problems/fibonacci-number/)

pub struct Solution;

impl Solution {
    pub fn fib(n: i32) -> i32 {
        assert!(n >= 0 && n <= 30);
        let r = (0..n).fold((0u64, 1), |(pp, p), _| (p, pp + p)).0;
        r as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn fib(n: i32) -> i32 {
        match n {
            0 | 1 => n,
            n => fib(n - 1) + fib(n - 2),
        }
    }

    #[test]
    fn test() {
        assert_eq!(fib(0), Solution::fib(0));
        assert_eq!(fib(1), Solution::fib(1));
        assert_eq!(fib(2), Solution::fib(2));
        assert_eq!(fib(3), Solution::fib(3));
        assert_eq!(fib(21), Solution::fib(21));
    }
}
