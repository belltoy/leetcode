//! # 231. 2的幂
//!
//! 难度 简单
//!
//! 给定一个整数，编写一个函数来判断它是否是 2 的幂次方。
//!
//! ## 示例 1:
//!
//! ```text
//! 输入: 1
//! 输出: true
//! 解释: 2^0 = 1
//! ```
//!
//! ## 示例 2:
//!
//! ```text
//! 输入: 16
//! 输出: true
//! 解释: 2^4 = 16
//! ```
//!
//! ## 示例 3:
//!
//! ```text
//! 输入: 218
//! 输出: false
//! ```
//!
//! See [leetcode](https://leetcode-cn.com/problems/power-of-two/)

pub struct Solution;

impl Solution {

    /// 2 的幂：大于 0，而且二进制表示中只有一个 1。
    ///
    /// n > 0 时才有可能是 2 的幂
    /// `n & (n - 1)` 相当于把最后一位的 `1` 变成 `0`，如果结果为 `0` 那就是 2 的幂，否则不是。
    pub fn is_power_of_two(n: i32) -> bool {
        n > 0 && n & (n - 1) == 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let t = |n| Solution::is_power_of_two(n);

        assert_eq!(false, t(-2147483648));
        assert_eq!(false, t(-2));
        assert_eq!(true, t(1));
        assert_eq!(true, t(16));
        assert_eq!(false, t(218));
    }
}
