//! # 50. Pow(x, n)
//!
//! 难度 中等
//!
//! 实现 pow(x, n) ，即计算 x 的 n 次幂函数。
//!
//! ## 示例 1:
//!
//! ```text
//! 输入: 2.00000, 10
//! 输出: 1024.00000
//! ```
//!
//! ## 示例 2:
//!
//! ```text
//! 输入: 2.10000, 3
//! 输出: 9.26100
//! ```
//!
//! ## 示例 3:
//!
//! ```text
//! 输入: 2.00000, -2
//! 输出: 0.25000
//! 解释: 2-2 = 1/22 = 1/4 = 0.25
//! ```
//!
//! ## 说明:
//!
//! - `-100.0 < x < 100.0`
//! - *n* 是 32 位有符号整数，其数值范围是 `[−231, 231 − 1]` 。
//!
//! See [leetcode](https://leetcode-cn.com/problems/powx-n/)

pub struct Solution;

impl Solution {

    /// 把 n 考虑成 2 进制表示，例如 100_1011 = 10_0000 + 1000 + 10 + 1
    /// 所以，x 的 n 次幂可以拆分成每一位 1 对应的数的次幂
    pub fn my_pow(x: f64, n: i32) -> f64 {
        if n >= 0 {
            do_pow(x, n as u32)
        } else {
            1.0 / do_pow(x, -n as u32)
        }
    }
}

fn do_pow(mut x: f64, mut n: u32) -> f64 {
    let mut ans = 1.0;

    while n > 0 {
        if n & 1 == 1 {
            ans *= x;
        }
        x *= x;
        n >>= 1;
    }
    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let cases = vec![
            (2.0_f64, 10),
            (2.1, 3),
            (2.0, -2),
            (0.0, 0),
        ];

        for (x, n) in cases {
            assert_eq!(x.powi(n), Solution::my_pow(x, n));
        }
    }
}
