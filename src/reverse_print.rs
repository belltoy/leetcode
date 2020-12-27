//! # 剑指 Offer 06. 从尾到头打印链表
//!
//! 难度 简单
//!
//! 输入一个链表的头节点，从尾到头反过来返回每个节点的值（用数组返回）。
//!
//! ## 示例 1：
//!
//! ```text
//! 输入：head = [1,3,2]
//! 输出：[2,3,1]
//! ```
//!
//! ## 限制：
//!
//! `0 <= 链表长度 <= 10000`
//!
//! See [leetcode](https://leetcode-cn.com/problems/cong-wei-dao-tou-da-yin-lian-biao-lcof/)

use crate::ListNode;

pub struct Solution;

impl Solution {
    pub fn reverse_print(head: Option<Box<ListNode>>) -> Vec<i32> {
        head.map(|mut head| {
            if head.next.is_some() {
                let mut v = Solution::reverse_print(head.next.take());
                v.push(head.val);
                v
            } else {
                vec![head.val]
            }
        }).unwrap_or(vec![])
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::list;

    #[test]
    fn test() {
        let cases = vec![
            (vec![2,3,1], list![1,3,2]),
            (vec![], list![]),
        ];
        for (expect, input) in cases {
            assert_eq!(expect, Solution::reverse_print(input));
        }
    }
}
