//! # 393. UTF-8 编码验证
//!
//! 难度 中等
//!
//! UTF-8 中的一个字符可能的长度为 1 到 4 字节，遵循以下的规则：
//!
//! 1. 对于 1 字节的字符，字节的第一位设为0，后面7位为这个符号的unicode码。
//! 2. 对于 n 字节的字符 (n > 1)，第一个字节的前 n 位都设为1，第 n+1 位设为0，后面字节的前两位一律设为10。剩下的没有提及的二进制位，全部为这个符号的unicode码。
//!
//! 这是 UTF-8 编码的工作方式：
//!
//! ```text
//!    Char. number range  |        UTF-8 octet sequence
//!       (hexadecimal)    |              (binary)
//!    --------------------+---------------------------------------------
//!    0000 0000-0000 007F | 0xxxxxxx
//!    0000 0080-0000 07FF | 110xxxxx 10xxxxxx
//!    0000 0800-0000 FFFF | 1110xxxx 10xxxxxx 10xxxxxx
//!    0001 0000-0010 FFFF | 11110xxx 10xxxxxx 10xxxxxx 10xxxxxx
//! ```
//!
//! 给定一个表示数据的整数数组，返回它是否为有效的 utf-8 编码。
//!
//! ## 注意:
//!
//! 输入是整数数组。只有每个整数的最低 8 个有效位用来存储数据。这意味着每个整数只表示 1 字节的数据。
//!
//! ## 示例 1:
//!
//! ```text
//! data = [197, 130, 1], 表示 8 位的序列: 11000101 10000010 00000001.
//!
//! 返回 true 。
//! 这是有效的 utf-8 编码，为一个2字节字符，跟着一个1字节字符。
//! ```
//!
//! ## 示例 2:
//!
//! ```text
//! data = [235, 140, 4], 表示 8 位的序列: 11101011 10001100 00000100.
//!
//! 返回 false 。
//! 前 3 位都是 1 ，第 4 位为 0 表示它是一个3字节字符。
//! 下一个字节是开头为 10 的延续字节，这是正确的。
//! 但第二个延续字节不以 10 开头，所以是不符合规则的。
//! ```
//!
//! See [leetcode](https://leetcode-cn.com/problems/utf-8-validation/)

pub struct Solution;

impl Solution {

    /// 支持 Pattern Matching 的语言做这种事很简单。
    ///
    /// 另外 Rust 的 Iterator 抽象很方便。
    ///
    /// 每位检查如果符合规则，要 `continue` 继续检查剩余字节，直到所有字节都符合，返回 `true`。否则，检查过程中只要遇到不符合规则的，就直接返回 `false`。
    ///
    /// 低位直接检查是否符合就行，要注意先要判断高位的情况，即开头超过 4 个 1 的字节是不符合的。
    pub fn valid_utf8(data: Vec<i32>) -> bool {
        // 取低 8 位，根据题目输入数据的限制，`as u8` 也行。
        // `fuse()` 保证到结尾后多次调用 `.next()` 依然返回 `None`
        let mut t = data.iter().map(|&i| i.to_le_bytes()[0]).fuse();
        let mut valid = true;
        while let (Some(n), true) = (t.next(), valid) {
            valid = match n {
                // 前缀越界，因为 UTF-8 只能是 1 到 4 字节，前缀不可能是 5 位或以上的 1 开头
                n if n & 0b1111_1000 == 0b1111_1000 => false,
                // 4 字节情况，取后三位检查
                n if n & 0b1111_0000 == 0b1111_0000 => match (t.next(), t.next(), t.next()) {
                    (Some(n1), Some(n2), Some(n3)) if n1 & n2 & n3 & 0b1000_0000 == 0b1000_0000 => true,
                    _ => false,
                }
                // 3 字节情况，取后两位检查
                n if n & 0b1110_0000 == 0b1110_0000 => match (t.next(), t.next()) {
                    (Some(n1), Some(n2)) if n1 & n2 & 0b1000_0000 == 0b1000_0000 => true,
                    _ => false,
                }
                // 2 字节情况，取后一位检查
                n if n & 0b1100_0000 == 0b1100_0000 => match t.next() {
                    Some(n1) if n1 & 0b1000_0000 == 0b1000_0000 => true,
                    _ => false,
                }
                // 1 字节情况
                // 只要是 `0xxx_xxxx` 以 `0` 开头就 OK
                // 注意，这里排除了 `10xx_xxxx` 开头的情况，这种情况是不符合规则的
                n if n & 0b1000_0000 == 0 => true,
                // 其它情况，均不符合规则
                _ => false,
            };
        }

        valid
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let cases = vec![
            (true, vec![
             vec![],
             vec![197, 130, 1],
            ]),

            (false, vec![
             vec![235, 140, 4],
             vec![0b1111_1111, 130, 1],
             vec![0b1100_0000, 0b0000_1111],
             vec![250,145,145,145,145],
             vec![248,130,130,130],
             vec![0b1011_0000, 130, 130],
            ]),
        ];
        for (expect, sub_cases) in cases {
            sub_cases.into_iter().map(Solution::valid_utf8).for_each(|x| assert_eq!(expect, x));
        }
    }
}
