//! # 397. 整数替换
//!
//! 难度 中等
//!
//! 给定一个正整数 `n` ，你可以做如下操作：
//!
//! 如果 `n` 是偶数，则用 `n / 2` 替换 `n` 。
//! 如果 `n` 是奇数，则可以用 `n + 1` 或 `n - 1` 替换 `n` 。
//! `n` 变为 `1` 所需的最小替换次数是多少？
//!
//!
//!
//! ## 示例 1：
//!
//! ```plain
//! 输入：n = 8
//! 输出：3
//! 解释：8 -> 4 -> 2 -> 1
//! ```
//!
//! ## 示例 2：
//!
//! ```plain
//! 输入：n = 7
//! 输出：4
//! 解释：7 -> 8 -> 4 -> 2 -> 1
//! 或 7 -> 6 -> 3 -> 2 -> 1
//! ```
//!
//! ## 示例 3：
//!
//! ```plain
//! 输入：n = 4
//! 输出：2
//! ```
//!
//!
//! ## 提示：
//!
//! * `1 <= n <= 231 - 1`
//!
//! See [leetcode](https://leetcode-cn.com/problems/integer-replacement/)

pub struct Solution;

impl Solution {
    pub fn integer_replacement(n: i32) -> i32 {
        let mut times = 0;
        let mut n: i64 = n as i64;
        while n > 1 {
            n = match n % 2 {
                0 => n / 2,
                _ => {
                    if n == 3 {
                        n - 1
                    } else {
                        match n & 0b0000_0000_0000_0011 {
                            0b0000_0000_0000_0011 => n + 1,
                            0b0000_0000_0000_0001 => n - 1,
                            _ => unreachable!(),
                        }
                    }
                }
            };
            times += 1;
        }
        times
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let t = |n| Solution::integer_replacement(n);
        assert_eq!(3, t(8));
        assert_eq!(4, t(7));
        assert_eq!(2, t(4));
    }
}
