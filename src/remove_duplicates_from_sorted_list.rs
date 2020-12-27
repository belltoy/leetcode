//! # 83. 删除排序链表中的重复元素
//!
//! 难度 简单
//!
//! 给定一个排序链表，删除所有重复的元素，使得每个元素只出现一次。
//!
//! ## 示例 1:
//!
//! ```text
//! 输入: 1->1->2
//! 输出: 1->2
//! ```
//!
//! ## 示例 2:
//!
//! ```text
//! 输入: 1->1->2->3->3
//! 输出: 1->2->3
//! ```
//!
//! See [leetcode](https://leetcode-cn.com/problems/remove-duplicates-from-sorted-list/)

use crate::ListNode;

pub struct Solution;

impl Solution {
    pub fn delete_duplicates(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut p = &mut head;
        while let Some(node) = p {
            if let Some(next) = node.next.as_mut() {
                if node.val == next.val {
                    node.next = next.next.take();
                }
            }
            p = &mut node.next;
        }
        head
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::list;

    #[test]
    fn test() {
        let cases = vec![
            (vec![1,2], list![1,1,2]),
            (vec![1,2,3], list![1,1,2,3,3,]),
            (vec![], list![]),
            (vec![1,2], list![1,2]),
        ];
        let t = |v| ListNode::into_vec(Solution::delete_duplicates(v));
        for (expect, input) in cases {
            assert_eq!(expect, t(input));
        }
    }
}
