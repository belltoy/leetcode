//! # 34. 在排序数组中查找元素的第一个和最后一个位置
//!
//! 难度 中等
//!
//! 给定一个按照升序排列的整数数组 nums，和一个目标值 target。找出给定目标值在数组中的开始位置和结束位置。
//!
//! 如果数组中不存在目标值 target，返回 [-1, -1]。
//!
//! ## 进阶：
//!
//! 你可以设计并实现时间复杂度为 O(log n) 的算法解决此问题吗？
//!
//!
//! ## 示例 1：
//!
//! ```
//! /// 输入：nums = [5,7,7,8,8,10], target = 8
//! /// 输出：[3,4]
//! use leetcode::search_range::Solution;
//! assert_eq!(vec![3i32, 4], Solution::search_range(vec![5,7,7,8,8,10], 8));
//! ```
//!
//! ## 示例 2：
//!
//! ```
//! /// 输入：nums = [5,7,7,8,8,10], target = 6
//! /// 输出：[-1,-1]
//! use leetcode::search_range::Solution;
//! assert_eq!(vec![-1i32, -1], Solution::search_range(vec![5,7,7,8,8,10], 6));
//! ```
//!
//! ## 示例 3：
//!
//! ```
//! /// 输入：nums = [], target = 0
//! /// 输出：[-1,-1]
//! use leetcode::search_range::Solution;
//! assert_eq!(vec![-1i32, -1], Solution::search_range(vec![], 0));
//! ```
//!
//! ## 提示：
//!
//! * `0 <= nums.length <= 105`
//! * `-109 <= nums[i] <= 109`
//! * nums 是一个非递减数组
//! * `-109 <= target <= 109`
//!
pub struct Solution;
impl Solution {

    pub fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
        match (binary_search_low(&nums[..], target), binary_search_high(&nums[..], target)) {
            (Some(lower), Some(upper)) => vec![lower as i32, upper as i32],
            _ => vec![-1, -1],
        }
    }
}

// TODO
fn binary_search_low(nums: &[i32], target: i32) -> Option<usize> {
    let mut len = nums.len();
    if len == 0 {
        return None;
    } else if len == 1 && nums[0] == target {
        return Some(0);
    }
    let mut base = 0;
    let mut res = None;

    while len > 1 {
        let half = len / 2;
        let mid = base + half;
        if len == 2 {
            if nums[base] == target {
                return Some(base);
            } else if nums[mid] == target {
                return Some(mid);
            }
        }
        base = if nums[mid] >= target {
            res = Some(mid);
            base
        } else {
            mid
        };
        len -= half;
    }
    res.and_then(|r| {
        if nums[r] == target {
            Some(r)
        } else {
            None
        }
    })
}

fn binary_search_high(nums: &[i32], target: i32) -> Option<usize> {
    let mut len = nums.len();
    if len == 0 {
        return None;
    } else if len == 1 && nums[0] == target {
        return Some(0);
    }

    let mut base = 0;
    let mut res = None;

    while len > 1 {
        let half = len / 2;
        let mid = base + half;

        if len == 2 {
            if nums[mid] == target {
                return Some(mid);
            } else if nums[base] == target {
                return Some(base);
            }
        }

        base = if nums[mid] > target {
            res = Some(base);
            base
        } else {
            mid
        };
        len -= half;
    }
    res.and_then(|r| {
        if nums[r] == target {
            Some(r)
        } else {
            None
        }
    })
}
