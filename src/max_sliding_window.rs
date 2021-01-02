//! # 239. 滑动窗口最大值
//!
//! 难度 困难
//!
//! 给你一个整数数组 nums，有一个大小为 k 的滑动窗口从数组的最左侧移动到数组的最右侧。你只可以看到在滑动窗口内的 k 个数字。滑动窗口每次只向右移动一位。
//!
//! 返回滑动窗口中的最大值。
//!
//!
//! ## 示例 1：
//!
//! ```text
//! 输入：nums = [1,3,-1,-3,5,3,6,7], k = 3
//! 输出：[3,3,5,5,6,7]
//! 解释：
//! 滑动窗口的位置                最大值
//! ---------------               -----
//! [1  3  -1] -3  5  3  6  7       3
//!  1 [3  -1  -3] 5  3  6  7       3
//!  1  3 [-1  -3  5] 3  6  7       5
//!  1  3  -1 [-3  5  3] 6  7       5
//!  1  3  -1  -3 [5  3  6] 7       6
//!  1  3  -1  -3  5 [3  6  7]      7
//! ```
//!
//! ## 示例 2：
//!
//! ```text
//! 输入：nums = [1], k = 1
//! 输出：[1]
//! ```
//!
//! ## 示例 3：
//!
//! ```text
//! 输入：nums = [1,-1], k = 1
//! 输出：[1,-1]
//! ```
//!
//! ## 示例 4：
//!
//! ```text
//! 输入：nums = [9,11], k = 2
//! 输出：[11]
//! ```
//!
//! ## 示例 5：
//!
//! ```text
//! 输入：nums = [4,-2], k = 2
//! 输出：[4]
//! ```
//!
//!
//! ## 提示：
//!
//! * `1 <= nums.length <= 10^5`
//! * `-104 <= nums[i] <= 10^4`
//! * `1 <= k <= nums.length`
//!
//! See [leetcode](https://leetcode-cn.com/problems/sliding-window-maximum/)
use std::collections::VecDeque;

pub struct Solution;

impl Solution {
    pub fn max_sliding_window(nums: Vec<i32>, k: i32) -> Vec<i32> {
        assert!(k >= 1 && k <= nums.len() as i32);
        let k = k as usize;
        let mut iter = nums.iter().enumerate();
        let queue = iter.by_ref().take(k - 1)
        .fold(VecDeque::new(), |mut queue, (i, &v)| {
            while let Some(&(_, last)) = queue.back() {
                if last < v {
                    queue.pop_back();
                } else {
                    break;
                }
            }
            queue.push_back((i, v));
            queue
        });

        iter.scan(queue, |queue, (i, &v)| {
            while let Some(&(i0, _)) = queue.front() {
                if i0 < i + 1 - k {
                    queue.pop_front();
                } else {
                    break;
                }
            }
            while let Some(&(_, last)) = queue.back() {
                if last < v {
                    queue.pop_back();
                } else {
                    break;
                }
            }
            queue.push_back((i, v));
            Some(queue.front().unwrap().1)
        })
        .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let cases = vec![
            ((vec![1,3,-1,-3,5,3,6,7], 3), vec![3,3,5,5,6,7]),
        ];

        for ((nums, k), expect) in cases {
            assert_eq!(expect, Solution::max_sliding_window(nums, k));
        }
    }
}
