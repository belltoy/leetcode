//! # 8. 字符串转换整数 (atoi)
//!
//! 难度 中等
//!
//! 请你来实现一个 atoi 函数，使其能将字符串转换成整数。
//!
//! 首先，该函数会根据需要丢弃无用的开头空格字符，直到寻找到第一个非空格的字符为止。接下来的转化规则如下：
//!
//! - 如果第一个非空字符为正或者负号时，则将该符号与之后面尽可能多的连续数字字符组合起来，形成一个有符号整数。
//! - 假如第一个非空字符是数字，则直接将其与之后连续的数字字符组合起来，形成一个整数。
//! - 该字符串在有效的整数部分之后也可能会存在多余的字符，那么这些字符可以被忽略，它们对函数不应该造成影响。
//!
//! 注意：假如该字符串中的第一个非空格字符不是一个有效整数字符、字符串为空或字符串仅包含空白字符时，则你的函数不需要进行转换，即无法进行有效转换。
//!
//! 在任何情况下，若函数不能进行有效的转换时，请返回 0 。
//!
//! 提示：
//!
//! - 本题中的空白字符只包括空格字符 ' ' 。
//! - 假设我们的环境只能存储 32 位大小的有符号整数，那么其数值范围为 [−2<sup>31</sup>,  2<sup>31</sup> − 1]。如果数值超过这个范围，请返回  INT_MAX (2<sup>31</sup> − 1) 或 INT_MIN (−2<sup>31</sup>) 。
//!
//!
//! ## 示例 1:
//!
//! ```text
//! 输入: "42"
//! 输出: 42
//! ```
//!
//! ## 示例 2:
//!
//! ```text
//! 输入: "   -42"
//! 输出: -42
//! 解释: 第一个非空白字符为 '-', 它是一个负号。
//!      我们尽可能将负号与后面所有连续出现的数字组合起来，最后得到 -42 。
//! ```
//!
//! ## 示例 3:
//!
//! ```text
//! 输入: "4193 with words"
//! 输出: 4193
//! 解释: 转换截止于数字 '3' ，因为它的下一个字符不为数字。
//! ```
//!
//! ## 示例 4:
//!
//! ```text
//! 输入: "words and 987"
//! 输出: 0
//! 解释: 第一个非空字符是 'w', 但它不是数字或正、负号。
//!      因此无法执行有效的转换。
//! ```
//!
//! ## 示例 5:
//!
//! ```text
//! 输入: "-91283472332"
//! 输出: -2147483648
//! 解释: 数字 "-91283472332" 超过 32 位有符号整数范围。
//!      因此返回 INT_MIN (−231) 。
//! ```
//!
//! See [leetcode](https://leetcode-cn.com/problems/string-to-integer-atoi/)

pub struct Solution;

impl Solution {

    /// FSM
    pub fn my_atoi(s: String) -> i32 {
        s.chars()
         .try_fold(State::new(), State::handle)
         .map_or_else(|n|n, State::into_ans)
    }
}

enum State {
    Start,
    Sign(i32),
    Num(i32),
}

impl State {
    fn new() -> Self{
        Self::Start
    }

    fn into_ans(self) -> i32 {
        match self {
            State::Num(n) => n,
            _ => 0,
        }
    }

    fn handle(self, c: char) -> Result<Self, i32> {
        match self {
            State::Start => match c {
                ' ' => Ok(State::Start),
                '+' => Ok(State::Sign(1)),
                '-' => Ok(State::Sign(-1)),
                c @ '0'..='9' => Ok(State::Num(c.to_digit(10).unwrap() as i32)),
                _ => Err(0),
            }
            State::Sign(sign) => match c {
                '0' => Ok(State::Sign(sign)),
                c @ '1'..='9' => Ok(State::Num(sign * c.to_digit(10).unwrap() as i32)),
                _ => Err(0),
            }
            State::Num(n) => match c {
                c @ '0'..='9' => {
                    if n >= 0 {
                        n.checked_mul(10)
                         .and_then(|n| n.checked_add(c.to_digit(10).unwrap() as i32))
                         .map(|n| State::Num(n))
                         .ok_or(i32::MAX)
                    } else {
                        n.checked_mul(10)
                         .and_then(|n| n.checked_sub(c.to_digit(10).unwrap() as i32))
                         .map(|n| State::Num(n))
                         .ok_or(i32::MIN)
                    }
                }
                _ => Err(n),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let cases: Vec<(i32, _)> = vec![
            (0, ""),
            (0, "+"),
            (0, "-"),
            (42, "42"),
            (-42, "   -42"),
            (-33, "  -0033z"),
            (0, "  - 0033z"),
            (4193, "4193 with words"),
            (0, "words and 987"),
            (2147483647, "2147483647"),
            (2147483647, "2147483648"),
            (2147483647, "9147483648"),
            (-2147483648, "-91283472332"),
            (-2147483648, "-2147483649"),
            (12345678, "  0000000000012345678"),
        ];

        for (expect, s) in cases {
            assert_eq!(expect, Solution::my_atoi(s.into()));
        }
    }
}
