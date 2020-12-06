//! # 394. 字符串解码
//!
//!
//! 难度 中等
//!
//! 给定一个经过编码的字符串，返回它解码后的字符串。
//!
//! 编码规则为: `k[encoded_string]`，表示其中方括号内部的 `encoded_string` 正好重复 `k` 次。注意 `k` 保证为正整数。
//!
//! 你可以认为输入字符串总是有效的；输入字符串中没有额外的空格，且输入的方括号总是符合格式要求的。
//!
//! 此外，你可以认为原始数据不包含数字，所有的数字只表示重复的次数 `k` ，例如不会出现像 `3a` 或 `2[4]` 的输入。
//!
//! ## 示例 1：
//!
//! ```plain
//! 输入：s = "3[a]2[bc]"
//! 输出："aaabcbc"
//! ```
//!
//! ## 示例 2：
//! ```plain
//!
//! 输入：s = "3[a2[c]]"
//! 输出："accaccacc"
//! ```
//!
//! ## 示例 3：
//! ```plain
//!
//! 输入：s = "2[abc]3[cd]ef"
//! 输出："abcabccdcdcdef"
//! ```
//!
//! ## 示例 4：
//! ```plain
//!
//! 输入：s = "abc3[cd]xyz"
//! 输出："abccdcdcdxyz"
//! ```
//! See [leetcode](https://leetcode-cn.com/problems/decode-string/)
pub struct Solution;
impl Solution {
    pub fn decode_string(s: String) -> String {
        Self::decode(&mut s.chars())
    }

    fn decode(mut iter: &mut std::str::Chars) -> String {
        let mut n = String::new();
        let mut s = String::new();
        while let Some(c) = iter.next() {
            match c {
                '[' => {
                    let r = Self::decode(&mut iter);
                    s.push_str(&r.repeat(n.parse::<usize>().unwrap_or(1)));
                    n = String::new(); // reset `n`, because `n` is right before `[`
                }
                ']' => {
                    break;
                }
                '0'..='9' => {
                    n.push(c);
                }
                c => {
                    s.push(c);
                }
            }
        }
        s.repeat(n.parse::<usize>().unwrap_or(1))
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test() {
        assert_eq!("aaabcbc", Solution::decode_string("3[a]2[bc]".into()));
        assert_eq!("accaccacc", Solution::decode_string("3[a2[c]]".into()));
        assert_eq!("abcabccdcdcdef", Solution::decode_string("2[abc]3[cd]ef".into()));
        assert_eq!("abccdcdcdxyz", Solution::decode_string("abc3[cd]xyz".into()));
        assert_eq!("zzzyypqjkjkefjkjkefjkjkefjkjkefyypqjkjkefjkjkefjkjkefjkjkefef", Solution::decode_string("3[z]2[2[y]pq4[2[jk]e1[f]]]ef".into()));
    }
}
