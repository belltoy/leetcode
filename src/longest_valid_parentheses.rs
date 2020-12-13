//! # 32. 最长有效括号
//!
//! 难度 困难
//!
//! 给定一个只包含 '(' 和 ')' 的字符串，找出最长的包含有效括号的子串的长度。
//!
//! ## 示例 1:
//!
//! ```text
//! 输入: "(()"
//! 输出: 2
//! 解释: 最长有效括号子串为 "()"
//! ```
//!
//! ## 示例 2:
//!
//! ```text
//! 输入: ")()())"
//! 输出: 4
//! 解释: 最长有效括号子串为 "()()"
//! ```
//!
//! See [leetcode](https://leetcode-cn.com/problems/longest-valid-parentheses/)
//!

use std::cmp::max;

/// Rust 的抽象层次很高，高级抽象，几乎零开销，函数式，高阶函数，
/// Pattern Matching 这些语法让程序写起来会很简洁，同时也不失去性能。
///
/// 主要是用 `Iterator::fold()` 的操作，不用栈，空间 O(1)，
/// 正反两次迭代，左右相等时记录最大的长度，反向也要执行一次。
/// 字符串的 `Chars` 本身就是一个双端的迭代器，实现了 `DoubleEndedIterator` 这个 trait，
/// `rev()` 操作也是零开销。这里为了避免反向遍历的时候左右计数条件要换，
/// 直接用一个 `map()` 把左右括号对换一下，所以计数的代码就可以直接复用了，
/// 只要一套从左到右的逻辑。
pub struct Solution;

impl Solution {

    pub fn longest_valid_parentheses(s: String) -> i32 {
        let from_left = longest(s.chars());
        let from_right = longest(s.chars().rev().map(|c| match c {
            '(' => ')',
            ')' => '(',
            _ => unreachable!(),
        }));
        max(from_left, from_right) as i32
    }
}

fn longest<T: Iterator<Item = char>>(chars: T) -> u32 {
    chars.fold((0, 0, 0), |(longest, left, right), c| match c {
        '(' => (longest, left + 1, right),
        ')' if left == right + 1 => (max(longest, left * 2), left, right + 1),
        ')' if left > right => (longest, left, right + 1),
        ')' => (longest, 0, 0),
        _ => unreachable!(),
    }).0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let cases = vec![
            (2, "(()(((()"),
            (4, ")()())"),
            (4, "(()()"),
            (6, "((((()))"),
            (0, ""),
            (0, "(((("),
            (0, ")(("),
            (0, ")))"),
            (0, ")))("),
            (2, "()(()"),
            (2, "(()"),
            (4, ")()())"),
            (18, "(((((())))()()()))"),
            (18, "(()))))))(())))(((((())))()()()))"),
        ];

        for (expect, s) in cases {
            assert_eq!(expect, Solution::longest_valid_parentheses(s.into()));
        }
    }
}
