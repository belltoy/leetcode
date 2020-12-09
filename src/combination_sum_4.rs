//! # 377. 组合总和 Ⅳ
//!
//! 难度 中等
//!
//! 给定一个由正整数组成且不存在重复数字的数组，找出和为给定目标正整数的组合的个数。
//!
//! ## 示例:
//!
//! ```text
//! nums = [1, 2, 3]
//! target = 4
//!
//! 所有可能的组合为：
//! (1, 1, 1, 1)
//! (1, 1, 2)
//! (1, 2, 1)
//! (1, 3)
//! (2, 1, 1)
//! (2, 2)
//! (3, 1)
//!
//! 请注意，顺序不同的序列被视作不同的组合。
//!
//! 因此输出为 7。
//! ```
//!
//! ## 进阶：
//!
//! - 如果给定的数组中含有负数会怎么样？
//! - 问题会产生什么变化？
//! - 我们需要在题目中添加什么限制来允许负数的出现？
//!
//! See [leetcode](https://leetcode-cn.com/problems/combination-sum-iv/)

pub struct Solution;

impl Solution {
    pub fn combination_sum4(nums: Vec<i32>, target: i32) -> i32 {
        debug_assert!(target >= 0);

        let mut s = Vec::with_capacity(target as usize + 1);
        s.push(1);                                           // 空集 1 个

        for i in 1..=target as usize {
            s.push(0);                                       // init s[i]
            for &n in nums.iter() {
                if n as usize <= i {
                    s[i] += s[i - n as usize];
                }
            }
        }
        s[target as usize]
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test() {
        let input = vec![1,2,3];
        assert_eq!(7, Solution::combination_sum4(input, 4));
    }
}
