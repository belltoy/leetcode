//! # 165. 比较版本号
//!
//! 难度 中等
//!
//! 给你两个版本号 `version1` 和 `version2` ，请你比较它们。
//!
//! 版本号由一个或多个修订号组成，各修订号由一个 `'.'` 连接。
//! 每个修订号由 **多位数字** 组成，可能包含 **前导零** 。
//! 每个版本号至少包含一个字符。修订号从左到右编号，下标从 0 开始，最左边的修订号下标为 0 ，下一个修订号下标为 1 ，以此类推。
//! 例如，`2.5.33` 和 `0.1` 都是有效的版本号。
//!
//! 比较版本号时，请按从左到右的顺序依次比较它们的修订号。比较修订号时，只需比较 **忽略任何前导零后的整数值** 。
//! 也就是说，修订号 `1` 和修订号 `001` 相等 。
//! 如果版本号没有指定某个下标处的修订号，则该修订号视为 `0` 。
//! 例如，版本 `1.0` 小于版本 `1.1` ，因为它们下标为 `0` 的修订号相同，而下标为 `1` 的修订号分别为 `0` 和 `1` ，`0 < 1` 。
//!
//! 返回规则如下：
//!
//! 如果 `version1 > version2` 返回 `1`，
//! 如果 `version1 < version2` 返回 `-1`，
//! 除此之外返回 `0`。
//!
//!
//! ## 示例 1：
//!
//! ```text
//! 输入：version1 = "1.01", version2 = "1.001"
//! 输出：0
//! 解释：忽略前导零，"01" 和 "001" 都表示相同的整数 "1"
//! ```
//!
//! ## 示例 2：
//!
//! ```text
//! 输入：version1 = "1.0", version2 = "1.0.0"
//! 输出：0
//! 解释：version1 没有指定下标为 2 的修订号，即视为 "0"
//! ```
//!
//! ## 示例 3：
//!
//! ```text
//! 输入：version1 = "0.1", version2 = "1.1"
//! 输出：-1
//! 解释：version1 中下标为 0 的修订号是 "0"，version2 中下标为 0 的修订号是 "1" 。0 < 1，所以 version1 < version2
//! ```
//!
//! ## 示例 4：
//!
//! ```text
//! 输入：version1 = "1.0.1", version2 = "1"
//! 输出：1
//! ```
//!
//! ## 示例 5：
//!
//! ```text
//! 输入：version1 = "7.5.2.4", version2 = "7.5.3"
//! 输出：-1
//! ```
//!
//!
//! ## 提示：
//!
//! - `1 <= version1.length, version2.length <= 500`
//! - `version1` 和 `version2` 仅包含数字和 `'.'`
//! - `version1` 和 `version2` 都是 **有效版本号**
//! - `version1` 和 `version2` 的所有修订号都可以存储在 **32 位整数** 中
//!
//! See [leetcode](https://leetcode-cn.com/problems/compare-version-numbers/)

pub struct Solution;

impl Solution {
    /// 使用 [`Iterator`](Iterator) 和 Pattern Matching 很简洁
    ///
    /// `s.split(".")` 拆成一个 Iterator，这种东西是 lazy 的，不需要调用 `collect()` 收集成 `Vec`，可以直接用 Iterator 的方式处理。
    ///
    /// 这里用了 `fuse()` 为了让 Iterator 可以持续调用 `next()` 直到两个 Iterator 都返回 `None` 才退出（说明比较完了）。
    ///
    /// 然后是 Pattern Matching 的方式，直接把所有情况列出。
    ///
    ///
    /// - 相同的情况，继续比较下一个
    /// - 不同的情况，可以直接返回结果
    /// - 其中一个为 `None` 的情况，说明这个字符串已经结束了，相当于后面都是 `0`，只要另一个还有非零的值就比它大（这里题目假设了输入都是有效的，所以不可能是负数）。
    /// - 最后全部比较完还没有差异，就是相同了，返回 `0`
    ///
    pub fn compare_version(version1: String, version2: String) -> i32 {
        // we can use `parse()` and `unwrap()` because the inputs are assumed valid
        let mut s1 = version1.split(".").fuse().map(|s| s.parse().unwrap());
        let mut s2 = version2.split(".").fuse().map(|s| s.parse().unwrap());
        loop {
            match (s1.next(), s2.next()) {
                (None, None) => break,
                (Some(0), None) | (None, Some(0)) => continue,
                (Some(_n1), None) => return 1,
                (None, Some(_n2)) => return -1,
                (Some(n1), Some(n2)) if n1 > n2 => return 1,
                (Some(n1), Some(n2)) if n1 < n2 => return -1,
                (Some(_n1), Some(_n2)) /*if n1 == n2*/ => continue,
            }
        }
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let t = |s1: &str, s2: &str| Solution::compare_version(s1.into(), s2.into());
        assert_eq!(0, t("1.01", "1.001"));
        assert_eq!(0, t("1.0", "1.0.0"));
        assert_eq!(-1, t("0.1", "1.1"));
        assert_eq!(1, t("1.0.1", "1"));
        assert_eq!(-1, t("7.5.2.4", "7.5.3"));
    }
}
