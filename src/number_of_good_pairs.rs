//! # 1512. 好数对的数目
//!
//! 难度 简单
//!
//! 给你一个整数数组 `nums` 。
//!
//! 如果一组数字 `(i,j)` 满足 `nums[i] == nums[j]` 且 `i < j` ，就可以认为这是一组 **好数对** 。
//!
//! 返回好数对的数目。
//!
//!
//! ## 示例 1：
//!
//! ```text
//! 输入：nums = [1,2,3,1,1,3]
//! 输出：4
//! 解释：有 4 组好数对，分别是 (0,3), (0,4), (3,4), (2,5) ，下标从 0 开始
//! ```
//!
//! ## 示例 2：
//!
//! ```text
//! 输入：nums = [1,1,1,1]
//! 输出：6
//! 解释：数组中的每组数字都是好数对
//! ```
//!
//! ## 示例 3：
//!
//! ```text
//! 输入：nums = [1,2,3]
//! 输出：0
//! ```
//!
//!
//! ## 提示：
//!
//! * `1 <= nums.length <= 100`
//! * `1 <= nums[i] <= 100`
//!
//! See [leetcode](https://leetcode-cn.com/problems/number-of-good-pairs/)

pub struct Solution;

impl Solution {
    pub fn num_identical_pairs(nums: Vec<i32>) -> i32 {
        debug_assert!(nums.len() >= 1 && nums.len() <= 100);
        nums.iter().fold(std::collections::HashMap::new(), |mut counts, &n| {
            counts.entry(n).and_modify(|v| *v += 1).or_insert(1);
            counts
        })
        .values()
        .map(|v| v * (v - 1) / 2)
        .sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let cases = vec![
            (4, vec![1,2,3,1,1,3]),
            (6, vec![1,1,1,1]),
            (0, vec![1,2,3]),
            (0, (1..=100).collect()),
        ];

        for (expect, input) in cases {
            assert_eq!(expect, Solution::num_identical_pairs(input));
        }
    }
}
