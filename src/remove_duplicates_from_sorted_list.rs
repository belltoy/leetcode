//! # 83. 删除排序链表中的重复元素
//!
//! 难度 简单
//!
//! 给定一个排序链表，删除所有重复的元素，使得每个元素只出现一次。
//!
//! ## 示例 1:
//!
//! ```plain
//! 输入: 1->1->2
//! 输出: 1->2
//! ```
//!
//! ## 示例 2:
//!
//! ```plain
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
            while let Some(next) = node.next.as_mut() {
                if node.val == next.val {
                    node.next = next.next.take();
                } else {
                    break;
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

    #[test]
    fn test() {
        let t = |v| ListNode::into_vec(Solution::delete_duplicates(ListNode::from_vec(v)));
        assert_eq!(vec![1,2], t(vec![1,1,2]));
        assert_eq!(vec![1,2,3], t(vec![1,1,2,3,3]));
        assert_eq!(vec![0i32;0], t(vec![]));
        assert_eq!(vec![1,2], t(vec![1,2]));
    }
}
