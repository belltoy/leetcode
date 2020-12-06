//! # 485. 最大连续1的个数
//!
//! 难度 简单
//!
//! 给定一个二进制数组， 计算其中最大连续1的个数。
//!
//! ## 示例 1:
//!
//! ```plain
//! 输入: [1,1,0,1,1,1]
//! 输出: 3
//! 解释: 开头的两位和最后的三位都是连续1，所以最大连续1的个数是 3。
//! ```
//!
//! ## 注意：
//!
//! - 输入的数组只包含 0 和 1。
//! - 输入数组的长度是正整数，且不超过 10,000。
//!
//! See [leetcode](https://leetcode-cn.com/problems/max-consecutive-ones/)
pub struct Solution;
impl Solution {
    pub fn find_max_consecutive_ones(nums: Vec<i32>) -> i32 {
        nums
            .split(|&x| x == 0)
            .map(|v| v.len())
            .max()
            // because split an empty vector will at least return an empty slice
            // which makes sense, so `max()` will always return `Some(T)`.
            // But the `max()` function signature returns `Option<T>`, we will use `unwrap_or(default)` for that.
            .unwrap_or(0) as i32
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test() {
        assert_eq!(3, Solution::find_max_consecutive_ones(vec![1, 1, 0, 1, 1, 1]));
        assert_eq!(0, Solution::find_max_consecutive_ones(vec![0, 0, 0, 0]));
        assert_eq!(0, Solution::find_max_consecutive_ones(vec![]));
    }
}
