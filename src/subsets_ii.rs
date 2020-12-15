//! # 90. 子集 II
//!
//! 难度 中等
//!
//! 给定一个可能包含重复元素的整数数组 nums，返回该数组所有可能的子集（幂集）。
//!
//! 说明：解集不能包含重复的子集。
//!
//! ## 示例:
//!
//! ```text
//! 输入: [1,2,2]
//! 输出:
//! [
//!   [2],
//!   [1],
//!   [1,2,2],
//!   [2,2],
//!   [1,2],
//!   []
//! ]
//! ```
//!
//! See [leetcode](https://leetcode-cn.com/problems/subsets-ii/)

pub struct Solution;

impl Solution {

    /// 回溯法
    pub fn subsets_with_dup(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut nums = nums;
        nums.sort();
        let mut ans = Vec::new();

        backtrack(&nums[..], &mut vec![], &mut ans);
        ans
    }
}

fn backtrack(nums: &[i32], v: &mut Vec<i32>, ans: &mut Vec<Vec<i32>>) {
    ans.push(v.clone());
    for i in 0..nums.len() {
        if i > 0 && nums[i] == nums[i - 1] {
            continue;
        }

        v.push(nums[i]);
        backtrack(&nums[(i + 1)..], v, ans);
        v.pop();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let cases: Vec<(Vec<i32>, Vec<Vec<i32>>)> = vec![
            (vec![1,2,2], vec![
                vec![2],
                vec![1],
                vec![1,2,2,],
                vec![2,2],
                vec![1,2],
                vec![],
            ]),
        ];

        for (nums, mut expect) in cases {
            let mut ans = Solution::subsets_with_dup(nums);
            ans.sort();
            expect.sort();
            assert_eq!(expect, ans);
        }
    }
}
