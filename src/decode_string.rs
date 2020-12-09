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
//! ```text
//! 输入：s = "3[a]2[bc]"
//! 输出："aaabcbc"
//! ```
//!
//! ## 示例 2：
//! ```text
//!
//! 输入：s = "3[a2[c]]"
//! 输出："accaccacc"
//! ```
//!
//! ## 示例 3：
//! ```text
//!
//! 输入：s = "2[abc]3[cd]ef"
//! 输出："abcabccdcdcdef"
//! ```
//!
//! ## 示例 4：
//! ```text
//!
//! 输入：s = "abc3[cd]xyz"
//! 输出："abccdcdcdxyz"
//! ```
//!
//! See [leetcode](https://leetcode-cn.com/problems/decode-string/)

pub struct Solution;

impl Solution {

    /// 递归
    pub fn decode_string(s: String) -> String {
        Self::decode(&mut s.chars())
    }

    // 因为题目说明「可以认为输入总是有效的」，所以这里不做有效性检查。
    //
    // * 读取每个字符，
    // * 遇到 `[` 用递归解码对应 `]` 之内的字符，repeat 之后拼接到 s 缓存，继续读取之后的字符
    // * 遇到 `]` 说明本次递归完成，退出 iter 循环，repeat 之后返回
    // * 遇到数字，加入 `number` 缓存
    // * 遇到其它字符，加入 `s` 缓存
    fn decode(mut iter: &mut std::str::Chars) -> String {
        let mut number = String::new(); // 数字缓存
        let mut s = String::new(); // 字符缓存
        while let Some(c) = iter.next() {
            match c {
                // 遇到 `[` 说明前面的数字读完了，递归解码 `]` 之前的字符，然后用数字重复
                // 同时，清空 `number`
                '[' => {
                    let r = Self::decode(&mut iter);
                    s.push_str(&r.repeat(number.parse::<usize>().unwrap_or(1)));
                    number = String::new(); // reset `number`, because `number` is right before `[`
                }
                // 遇到 `]` 说明读完了 `[]` 里的字符，在一次递归里这里已经完成，退出 iter
                // 循环，repeat 之后返回
                ']' => {
                    break;
                }
                // 遇到数字，加入 `number` 之后
                '0'..='9' => {
                    number.push(c);
                }
                // 遇到其它字符，加入 `s` 之后
                c => {
                    s.push(c);
                }
            }
        }

        // 本次递归读取完成，repeat 之后返回
        s.repeat(number.parse::<usize>().unwrap_or(1))
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
