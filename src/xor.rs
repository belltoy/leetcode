//! # 剑指 Offer 56 - I. 数组中数字出现的次数
//!
//! 一个整型数组 nums 里除两个数字之外，其他数字都出现了两次。请写程序找出这两个只出现一次的数字。要求时间复杂度是O(n)，空间复杂度是O(1)。
//!
//! 难度：中等
//!
//! ## 示例 1：
//!
//! > 输入：nums = [4,1,4,6]
//! > 输出：[1,6] 或 [6,1]
//!
//! ## 示例 2：
//!
//! > 输入：nums = [1,2,10,4,1,4,3,3]
//! > 输出：[2,10] 或 [10,2]
//!  
//!
//! ## 限制：
//!
//! 2 <= nums.length <= 10000
//!

pub struct Solution;

impl Solution {

    /// - 异或运算，消除出现了再次的数字，剩下就是要找的目标数字的异或结果 a ^ b
    /// - 再通过区分最低位的 1 把数组分成两部分，即每个部分只会包含一个目标数字。每个部分异或运算，可以找出这部分里的目标数字
    /// - 另一个目标数字可以用同样的方式找出
    /// - 更简单的方式是直接把已经找出的一个目标数字写之前两个目标异或的结果进行异或操作 (a ^ b ^ a = b)。
    pub fn single_numbers(nums: Vec<i32>) -> Vec<i32> {
        assert!(nums.len() >= 2 && nums.len() <= 10000);

        let xor = nums.iter().fold(0, |xor, x| xor ^ x);

        let mut helper = 1;
        while xor & helper == 0 {
            helper <<= 1;
        }
        let a = nums.iter().filter(|&x| x & helper == 0).fold(0, |a, x| a ^ x);
        vec![a, xor ^ a]
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test() {
        let expect: &[i32] = &[1,6][..];
        let mut result = Solution::single_numbers(vec![4,1,4,6]);
        result.sort();
        assert_eq!(expect, result);

        let expect: &[i32] = &[2,10][..];
        let mut result = Solution::single_numbers(vec![1,2,10,4,1,4,3,3]);
        result.sort();
        assert_eq!(expect, result);
    }
}
