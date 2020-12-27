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

use crate::ListNode;

pub struct Solution;

impl Solution {
    pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        Self::iterate(head)
    }

    #[inline]
    fn iterate(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        head.map(|mut new| {
            let mut curr = new.next.take();
            while let Some(mut node) = curr {
                curr = node.next.replace(new);
                new = node;
            }
            new
        })
    }

    #[inline]
    fn recursion(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        head.map(|mut head| {
            if head.next.is_none() {
                head
            } else {
                Self::recur(head.next.take(), head)
            }
        })
    }

    fn recur(curr: Option<Box<ListNode>>, pre: Box<ListNode>) -> Box<ListNode> {
        if let Some(mut curr) = curr {
            Self::recur(curr.next.replace(pre), curr)
        } else {
            pre
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::list;

    #[test]
    fn test() {
        let cases = vec![
            (vec![], list![]),
            (vec![1,2], list![2,1]),
            (vec![5,4,3,2,1], list![1,2,3,4,5]),
        ];
        let t1 = |v| ListNode::into_vec(Solution::iterate(v));
        let t2 = |v| ListNode::into_vec(Solution::recursion(v));

        for (expect, head) in cases {
            assert_eq!(expect, t1(head.clone()));
            assert_eq!(expect, t2(head));
        }
    }
}
