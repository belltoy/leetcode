//! # 剑指 Offer 10- I. 斐波那契数列
//!
//! 难度 简单
//!
//! 写一个函数，输入 n ，求斐波那契（Fibonacci）数列的第 n 项。斐波那契数列的定义如下：
//!
//! ```text
//! F(0) = 0,   F(1) = 1
//! F(N) = F(N - 1) + F(N - 2), 其中 N > 1.
//! ```
//!
//! 斐波那契数列由 0 和 1 开始，之后的斐波那契数就是由之前的两数相加而得出。
//!
//! 答案需要取模 1e9+7（1000000007），如计算初始结果为：1000000008，请返回 1。
//!
//!
//! ## 示例 1：
//!
//! ```text
//! 输入：n = 2
//! 输出：1
//! ```
//!
//! ## 示例 2：
//!
//! ```text
//! 输入：n = 5
//! 输出：5
//! ```
//!
//!
//! ## 提示：
//!
//! * `0 <= n <= 100`
//!
//! See [leetcode](https://leetcode-cn.com/problems/fei-bo-na-qi-shu-lie-lcof/)

pub struct Solution;

impl Solution {
    pub fn fib(n: i32) -> i32 {
        assert!(n >= 0 && n <= 100);
        let r = (0..n).fold((0u128, 1), |(pp, p), _| (p, pp + p)).0 % 1_000_000_007;
        r as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const P: i32 = 1_000_000_007;

    fn fib(n: i32) -> i32 {
        match n {
            0 | 1 => n,
            n => (fib(n - 1) % P + fib(n - 2) % P) % P,
        }
    }

    #[test]
    fn test() {
        assert_eq!(fib(0), Solution::fib(0));
        assert_eq!(fib(1), Solution::fib(1));
        assert_eq!(fib(2), Solution::fib(2));
        assert_eq!(fib(3), Solution::fib(3));
        assert_eq!(fib(21), Solution::fib(21));
        // assert_eq!(fib(44), Solution::fib(44));
        // assert_eq!(701408733, fib(44));
        assert_eq!(701408733, Solution::fib(44));
        assert_eq!(583861472, Solution::fib(55));
        assert_eq!(687995182, Solution::fib(100));
    }
}
