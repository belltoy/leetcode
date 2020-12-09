//! # 645. 错误的集合
//!
//! 难度 简单
//!
//! 集合 S 包含从1到 n 的整数。不幸的是，因为数据错误，导致集合里面某一个元素复制了成了集合里面的另外一个元素的值，导致集合丢失了一个整数并且有一个元素重复。
//!
//! 给定一个数组 nums 代表了集合 S 发生错误后的结果。你的任务是首先寻找到重复出现的整数，再找到丢失的整数，将它们以数组的形式返回。
//!
//! ## 示例 1:
//!
//! ```text
//! 输入: nums = [1,2,2,4]
//! 输出: [2,3]
//! ```
//!
//! ## 注意:
//!
//! - 给定数组的长度范围是 `[2, 10000]`。
//! - 给定的数组是无序的。
//!
//! See [leetcode](https://leetcode-cn.com/problems/set-mismatch/)

/// 异或操作
pub struct Solution;
impl Solution {
    pub fn find_error_nums(nums: Vec<i32>) -> Vec<i32> {
        let len = nums.len();
        let xor = (1..len as i32 + 1).fold(0, |xor, x| xor ^ x);
        let xor = nums.iter().fold(xor, |xor, x| xor ^ x);

        let mut helper = 1;
        while xor & helper == 0 {
            helper <<= 1;
        }

        let mut xor0 = 0;
        let mut xor1 = 0;
        (1..len as i32 + 1).chain(nums.iter().map(|&x| x)).for_each(|x| {
            if x & helper == 0 {
                xor0 ^= x;
            } else {
                xor1 ^= x;
            }
        });
        match nums.iter().any(|&x| x == xor0) {
            true => vec![xor0, xor1],
            false => vec![xor1, xor0],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test() {
        assert_eq!(&[2,3][..], Solution::find_error_nums(vec![1,2,2,4]));
    }
}
