//! # 400. 第N个数字
//!
//! 难度 中等
//!
//! 在无限的整数序列 `1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, ...` 中找到第 `n` 个数字。
//!
//! ## 注意:
//!
//! `n` 是正数且在 32 位整数范围内 ( `n < 231` )。
//!
//! ## 示例 1:
//!
//! ```plain
//! 输入:
//! 3
//!
//! 输出:
//! 3
//! ```
//!
//! ## 示例 2:
//!
//! ```plain
//! 输入:
//! 11
//!
//! 输出:
//! 0
//!
//! 说明:
//!
//! 第11个数字在序列 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, ... 里是0，它是10的一部分。
//! ```
//!
//! See [leetcode](https://leetcode-cn.com/problems/nth-digit/)

pub struct Solution;

impl Solution {
    pub fn find_nth_digit(n: i32) -> i32 {
        let mut rest = n as i64;
        let mut range: i64 = 9;
        let mut digits: i32 = 1;

        while rest - range * digits as i64 > 0 {
            rest = rest - range * digits as i64;
            digits += 1;
            range = range * 10;
        }
        let rest = rest as i32;

        let base = if digits > 1 {
            let mut base = 1;
            for _ in 1..digits {
                base *= 10;
            }
            base
        } else {
            0
        };

        if digits == 1 {
            return rest;
        }

        let mut target = rest / digits + base;
        let rem = rest % digits;
        if rem == 0 {
            target = target - 1;
            return target % 10;
        }

        let mut x = target;
        let mut y = 0;
        for _ in 0..(digits + 1 - rem) {
            y = x % 10;
            x = x / 10;
        }
        y
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let t = |n| Solution::find_nth_digit(n);
        assert_eq!(3, t(3));
        assert_eq!(0, t(11));
    }
}
