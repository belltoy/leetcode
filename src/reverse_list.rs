//! # 206. 反转链表
//!
//! 难度 简单
//!
//! 反转一个单链表。
//!
//! ## 示例:
//!
//! ```plain
//! 输入: 1->2->3->4->5->NULL
//! 输出: 5->4->3->2->1->NULL
//! ```
//!
//! ## 进阶:
//!
//! 你可以迭代或递归地反转链表。你能否用两种方法解决这道题？
//!
//! See [leetcode](https://leetcode-cn.com/problems/reverse-linked-list/)
//!

use crate::util::ListNode;

pub struct Solution;

impl Solution {
    pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut curr = head;
        let mut prev = None;
        while let Some(ref mut node) = curr {
            let t = node.next.take();
            node.next = prev;
            prev = curr;
            curr = t;
        }
        prev
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let t = |v| ListNode::into_vec(Solution::reverse_list(ListNode::from_vec(v)));
        assert_eq!(vec![5,4,3,2,1], t(vec![1,2,3,4,5]));
    }
}
