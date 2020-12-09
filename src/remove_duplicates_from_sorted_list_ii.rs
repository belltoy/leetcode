//! # 82. 删除排序链表中的重复元素 II
//!
//! 难度 中等
//!
//! 给定一个排序链表，删除所有含有重复数字的节点，只保留原始链表中 没有重复出现 的数字。
//!
//! ## 示例 1:
//!
//! ```plain
//! 输入: 1->2->3->3->4->4->5
//! 输出: 1->2->5
//! ```
//!
//! ## 示例 2:
//!
//! ```plain
//! 输入: 1->1->1->2->3
//! 输出: 2->3
//! ```
//!
//! See [leetcode](https://leetcode-cn.com/problems/remove-duplicates-from-sorted-list-ii/)
//!

use crate::ListNode;

pub struct Solution;

impl Solution {
    pub fn delete_duplicates(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        if head.is_none() || head.as_ref().unwrap().next.is_none() {
            return head;
        }

        if head.as_ref().unwrap().val == head.as_ref().unwrap().next.as_ref().unwrap().val {
            while head.is_some() && head.as_mut().unwrap().next.is_some() && head.as_mut().unwrap().next.as_mut().unwrap().val == head.as_mut().unwrap().val {
                head = head.as_mut().unwrap().next.take();
            }
            return Solution::delete_duplicates(head.and_then(|mut head| head.next.take()));
        } else {
            head.as_mut().unwrap().next = Solution::delete_duplicates(head.as_mut().unwrap().next.take());
            head
        }
    }

    pub fn delete_duplicates_2(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        head.and_then(|mut head| {
            let mut next = head.next.take();
            if next.is_none() {
                return head.into();
            }

            if next.as_ref().unwrap().val == head.val {
                while next.is_some() && head.val == next.as_ref().unwrap().val {
                    head = next.unwrap();
                    next = head.next.take();
                }
                Solution::delete_duplicates_2(next)
            } else {
                head.next = Solution::delete_duplicates_2(next);
                head.into()
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let t = |t| ListNode::into_vec(Solution::delete_duplicates(ListNode::from_vec(t)));
        let t2 = |t| ListNode::into_vec(Solution::delete_duplicates_2(ListNode::from_vec(t)));

        assert_eq!(vec![1,2,5], t(vec![1,2,3,3,4,4,5]));
        assert_eq!(vec![2,3], t(vec![1,1,1,2,3]));
        assert_eq!(vec![0i32;0], t(vec![]));

        assert_eq!(vec![1,2,5], t2(vec![1,2,3,3,4,4,5]));
        assert_eq!(vec![2,3], t2(vec![1,1,1,2,3]));
        assert_eq!(vec![0i32;0], t2(vec![]));
    }
}
