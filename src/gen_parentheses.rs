//! # 22. 括号生成
//!
//! 难度 中等
//!
//! 数字 n 代表生成括号的对数，请你设计一个函数，用于能够生成所有可能的并且 有效的 括号组合。
//!
//! ## 示例：
//!
//! ```plain
//! 输入：n = 3
//! 输出：[
//!        "((()))",
//!        "(()())",
//!        "(())()",
//!        "()(())",
//!        "()()()"
//!      ]
//! ```
//!
//! See [leetcode](https://leetcode-cn.com/problems/generate-parentheses/)
pub struct Solution;
impl Solution {
    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        let mut res = vec![];
        Self::dfs(0, 0, n, &mut String::new(), &mut res);
        res
    }

    fn dfs(left: i32, right: i32, n: i32, chars: &mut String, res: &mut Vec<String>) {
        if chars.len() == n as usize * 2 {
            res.push(chars.clone());
            return;
        }
        if left < n {
            chars.push('(');
            Self::dfs(left + 1, right, n, chars, res);
            chars.pop();
        }
        if right < left {
            chars.push(')');
            Self::dfs(left, right + 1, n, chars, res);
            chars.pop();
        }
    }
}
