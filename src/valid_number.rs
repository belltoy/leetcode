//! # 65. 有效数字
//!
//! 难度 困难
//!
//! 验证给定的字符串是否可以解释为十进制数字。
//!
//! 例如:
//!
//! - `"0"` => `true`
//! - `" 0.1 "` => `true`
//! - `"abc"` => `false`
//! - `"1 a"` => `false`
//! - `"2e10"` => `true`
//! - `" -90e3   "` => `true`
//! - `" 1e"` => `false`
//! - `"e3"` => `false`
//! - `" 6e-1"` => `true`
//! - `" 99e2.5 "` => `false`
//! - `"53.5e93"` => `true`
//! - `" --6 "` => `false`
//! - `"-+3"` => `false`
//! - `"95a54e53"` => `false`
//!
//! 说明: 我们有意将问题陈述地比较模糊。在实现代码之前，你应当事先思考所有可能的情况。这里给出一份可能存在于有效十进制数字中的字符列表：
//!
//! - 数字 0-9
//! - 指数 - "e"
//! - 正/负号 - "+"/"-"
//! - 小数点 - "."
//!
//! 当然，在输入中，这些字符的上下文也很重要。
//!
//! See [leetcode](https://leetcode-cn.com/problems/valid-number/)

/// DFA
///
/// `enum` + Pattern Matching 实现状态机，再加上 `Iterator`/`Result` 的高阶函数组合工具，
/// 代码可读性比写个表找下标不知道高哪里去了。
///
/// 另外，开始不需要 `trim()`，一是因为这个操作结合起来整个实现并没有这里的实现高效，
/// 二是内部调用了 `char:is_whitespace()` 不止判断了空格，还有其它空白字符，不一定符合题目的测试用例。
pub struct Solution;

impl Solution {
    pub fn is_number(s: String) -> bool {
        s.chars()
         .try_fold(State::new(), |s, c| s.handle(c))
         .map_or(false, |s| s.is_valid())
    }
}

type Result = std::result::Result<State, ()>;

enum State {
    Start,
    Sign,
    Integer,
    Dot,
    EmptyDot,
    Decimal,
    E,
    ExpSign,
    Exponent,
    End,
}

impl State {
    fn new() -> Self {
        State::Start
    }

    fn is_valid(&self) -> bool {
        use State::*;
        match self {
            Start | Sign | E | ExpSign | EmptyDot => false,
            _ => true,
        }
    }

    fn handle(self, c: char) -> Result {
        use State::*;
        match self {
            Start => match c {
                ' ' => Ok(Start),
                '+' | '-' => Ok(Sign),
                '0'..='9' => Ok(Integer),
                '.' => Ok(EmptyDot),
                _ => Err(()),
            }
            Sign => match c {
                '0'..='9' => Ok(Integer),
                '.' => Ok(EmptyDot),
                _ => Err(()),
            }
            Integer => match c {
                '0'..='9' => Ok(Integer),
                '.' => Ok(Dot),
                'e' => Ok(E),
                ' ' => Ok(End),
                _ => Err(()),
            }
            EmptyDot => match c {
                '0'..='9' => Ok(Decimal), // "  .1" or "  +.1"
                _ => Err(()),
            }
            Dot => match c {
                '0'..='9' => Ok(Decimal),
                'e' => Ok(E),   // "46.e3"
                ' ' => Ok(End),
                _ => Err(()),
            }
            Decimal => match c {
                '0'..='9' => Ok(Decimal),
                'e' => Ok(E),
                ' ' => Ok(End),
                _ => Err(()),
            }
            E => match c {
                '+' | '-' => Ok(ExpSign),
                '0'..='9' => Ok(Exponent),
                _ => Err(()),
            }
            ExpSign => match c {
                '0'..='9' => Ok(Exponent),
                _ => Err(()),
            }
            Exponent => match c {
                '0'..='9' => Ok(Exponent),
                ' ' => Ok(End),
                _ => Err(()),
            }
            End => match c {
                ' ' => Ok(End),
                _ => Err(()),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn test() {
        let cases = (
            // false
            vec![
                "+.",
                ".",
                "1.0e4.5",
                "-000111.xxaslkdfjaskldfj",
                "12 3",
                "1a3",
                "",
                "     ",
                "46.e",
                "1e",
                "+",
                "-",
                "+.",
                "-.",
            ],
            // true
            vec![
                "123",
                " 123 ",
                "0",
                "0123",  //Cannot agree
                "00",  //Cannot agree
                "-10",
                "-0",
                "123.5",
                "123.000000",
                "-500.777",
                "0.0000001",
                "0.00000",
                "0.",  //Cannot be more disagree!!!
                "00.5",  //Strongly cannot agree
                "123e1",
                "1.23e10",
                "0.5e-10",
                "0.5e04",
                ".1", //Ok, if you say so
                "2e0",  //Really?!
                "+.8",
                " 005047e+6",  //Damn = =|||
                "-123.456e-789",
                "46.e3",
            ]);

        let cases = cases.0.iter().map(|s| (false, s)).chain(cases.1.iter().map(|s| (true, s)));
        for (expect, &s) in cases {
            assert_eq!(expect, Solution::is_number(s.into()),
                "expect is_number(\"{}\") = {}, but got {}, try parse: {:?}", s, expect, !expect, f32::from_str(s.trim()));
        }
    }
}
