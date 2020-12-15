//! # 78. 子集
//!
//! 难度 中等
//!
//! 给定一组不含重复元素的整数数组 nums，返回该数组所有可能的子集（幂集）。
//!
//! 说明：解集不能包含重复的子集。
//!
//! ## 示例:
//!
//! ```text
//! 输入: nums = [1,2,3]
//! 输出:
//! [
//!   [3],
//!   [1],
//!   [2],
//!   [1,2,3],
//!   [1,3],
//!   [2,3],
//!   [1,2],
//!   []
//! ]
//! ```
//!
//! See [leetcode](https://leetcode-cn.com/problems/subsets/)
//! and [more](https://leetcode-cn.com/problems/subsets/solution/rust-1-xing-dai-ma-gao-jie-han-shu-han-s-2gl2/)

pub struct Solution;

impl Solution {
    pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
        (0..(1 << nums.len())).map(|mask| {
            nums.iter().enumerate()
                // or use one `filter_map`
                .map(|(i, v)| (mask & (1 << i), v))
                .filter(|(i, _)| *i != 0)
                .map(|(_, &v)| v)
                .collect()
        }).collect()
    }
}

// let mut ans = Vec::new();
// backtrack(&nums[..], &mut vec![], &mut ans);
// ans
fn backtrack(nums: &[i32], subset: &mut Vec<i32>, ans: &mut Vec<Vec<i32>>) {
    if nums.len() == 0 {
        ans.push(subset.clone());
        return;
    }
    subset.push(nums[0]);
    backtrack(&nums[1..], subset, ans);
    subset.pop();
    backtrack(&nums[1..], subset, ans);
}

// producer(&nums).collect()
fn producer<'a>(nums: &'a [i32]) -> impl Iterator<Item = Vec<i32>> + 'a {
    (0..(1 << nums.len())).map(move |mask| {
        nums.iter().enumerate().filter_map(|(i, &v)| {
            if mask & (1 << i) != 0 {
                Some(v)
            } else {
                None
            }
        }).collect()
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let cases: Vec<(Vec<i32>, Vec<Vec<i32>>)> = vec![
            (vec![1,2,3], vec![
                vec![3],
                vec![1],
                vec![2],
                vec![1,2,3],
                vec![1,3],
                vec![2,3],
                vec![1,2],
                vec![]
            ]),
        ];

        for (nums, mut expect) in cases {
            expect.sort();
            let mut ans = Solution::subsets(nums);
            ans.sort();
            assert_eq!(expect, ans);
        }
    }
}
