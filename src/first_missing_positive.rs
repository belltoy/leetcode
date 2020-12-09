//! # 41. 缺失的第一个正数
//!
//! 难度 困难
//!
//! 给你一个未排序的整数数组，请你找出其中没有出现的最小的正整数。
//!
//!
//!
//! ## 示例 1:
//!
//! ```plain
//! 输入: [1,2,0]
//! 输出: 3
//! ```
//!
//! ## 示例 2:
//!
//! ```plain
//! 输入: [3,4,-1,1]
//! 输出: 2
//! ```
//!
//! ## 示例 3:
//!
//! ```plain
//! 输入: [7,8,9,11,12]
//! 输出: 1
//! ```
//!
//! See [leetcode](https://leetcode-cn.com/problems/first-missing-positive/)

pub struct Solution;

impl Solution {
    pub fn first_missing_positive(nums: Vec<i32>) -> i32 {
        let mut nums = nums;
        let len = nums.len() as i32;
        for i in 0..nums.len() {
            while nums[i] > 0 && nums[i] <= len && nums[nums[i] as usize - 1] != nums[i] {
                let v = nums[i] as usize - 1;
                nums.swap(i, v);
            }
        }

        for i in 0..nums.len() {
            if nums[i] != i as i32 + 1 {
                return i as i32 + 1;
            }
        }
        return len + 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let t = |n| Solution::first_missing_positive(n);
        assert_eq!(3, t(vec![1,2,0]));
        assert_eq!(2, t(vec![3,4,-1,1]));
        assert_eq!(1, t(vec![7,8,9,11,12]));
        assert_eq!(6, t(vec![1,2,3,4,5]));
    }
}
